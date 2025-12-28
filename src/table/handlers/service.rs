use crate::table::{
    models::{ColumnTable, RowTable},
    value::{ColumnType, Value},
};
use prettytable::{Attr, color};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTable {
    pub column: Vec<ColumnTable>,
    pub row: Vec<RowTable>,
    pub _increment: i64,
    pub primary_index: Option<usize>,
}

impl DataTable {
    pub fn new() -> Self {
        Self {
            column: Vec::new(),
            row: Vec::new(),
            primary_index: None,
            _increment: 0,
        }
    }

    // Set primary
    pub fn set_primary(&mut self, colname: &str) -> Result<(), String> {
        let index = self
            .column
            .iter()
            .position(|c| c.colname == colname)
            .ok_or_else(|| format!("SET_PRIMARY: kolom `{}` tidak ditemukan", colname))?;

        // Jika sudah primary → skip
        if self.primary_index == Some(index) {
            return Ok(());
        }

        // Validasi ketat jika ada row
        if !self.row.is_empty() {
            self._validate_primary_candidate(index)?;
        }

        // Reset primary lama
        for col in &mut self.column {
            col.is_primary = false;
        }

        // Pasang primary baru
        self.column[index].is_primary = true;
        self.primary_index = Some(index);

        Ok(())
    }

    // Tambah kolom
    pub fn add_column(&mut self, columns: Vec<(&str, ColumnType)>) -> Result<(), String> {
        for (colname, coltype) in columns {
            if self.column.iter().any(|c| c.colname == colname) {
                return Err(format!("ADD_COLUMN: kolom `{}` sudah ada", colname));
            }

            self.column.push(ColumnTable {
                colname: colname.into(),
                coltype,
                is_primary: false,
                _is_auto_increment: false,
            });
        }

        // normalize jumlah nilai pada baris sesuai jumlah kolom
        let col_len = self.column.len();
        for row in &mut self.row {
            while row.value.len() < col_len {
                row.value.push(Value::Empty); // tambah hanya jika row terlalu pendek
            }
        }
        // ketika tambah kolom baru maka baris sebelumnya akan otomatis isi Empty

        Ok(())
    }

    // Tambah row
    pub fn add_row(&mut self, mut value: Vec<Value>) -> Result<(), String> {
        if self.primary_index.is_none() {
            return Err("ADD_ROW: primary belum ditentukan".into());
        }

        let col_len = self.column.len();

        if value.len() > col_len {
            return Err(format!(
                "ADD_ROW: nilai berlebih ({}), kolom hanya ({})",
                value.len(),
                col_len
            ));
        }

        // isi kekurangan dengan Empty
        while value.len() < col_len {
            value.push(Value::Empty);
        }

        // VALIDASI TIPE
        for (i, val) in value.iter().enumerate() {
            let col = &self.column[i];
            if !Self::_validate_type_column_and_row(&col.coltype, val) {
                return Err(format!(
                    "ADD_ROW: tipe tidak cocok pada kolom `{}`",
                    col.colname
                ));
            }
        }

        self.row.push(RowTable { value });
        Ok(())
    }

    // Update value berdasarkan kondisi
    pub fn set_value_where(
        &mut self,
        cond_col: &str,
        cond_value: Value,
        target_col: &str,
        new_value: Value,
    ) -> Result<(), String> {
        let cond_index = self
            .column
            .iter()
            .position(|c| c.colname == cond_col)
            .ok_or_else(|| format!("SET_VALUE: kolom kondisi `{}` tidak ditemukan", cond_col))?;

        let target_index = self
            .column
            .iter()
            .position(|c| c.colname == target_col)
            .ok_or_else(|| format!("SET_VALUE: kolom target `{}` tidak ditemukan", target_col))?;

        let target_col_def = &self.column[target_index];
        if !Self::_validate_type_column_and_row(&target_col_def.coltype, &new_value) {
            return Err(format!(
                "SET_VALUE: tipe tidak cocok untuk kolom `{}`",
                target_col_def.colname
            ));
        }

        // Validasi ketat jika target adalah primary
        if self.primary_index == Some(target_index) {
            if matches!(new_value, Value::Empty) {
                return Err("SET_VALUE: primary tidak boleh Empty".into());
            }
            for row in &self.row {
                if row.value[target_index] == new_value {
                    return Err("SET_VALUE: nilai primary duplikat".into());
                }
            }
        }

        let mut found = false;
        for row in &mut self.row {
            if row.value[cond_index] == cond_value {
                row.value[target_index] = new_value.clone();
                found = true;
            }
        }

        if !found {
            return Err("SET_VALUE: tidak ada baris yang cocok dengan kondisi".into());
        }

        Ok(())
    }
    // Hapus kolom
    pub fn remove_column(&mut self, colname: &str) -> Result<(), String> {
        let index = self
            .column
            .iter()
            .position(|c| c.colname == colname)
            .ok_or_else(|| format!("HAPUS_KOLOM: kolom `{}` tidak ditemukan", colname))?;

        // Cegah hapus primary
        if self.primary_index == Some(index) {
            return Err(format!(
                "HAPUS_KOLOM: kolom `{}` adalah primary, tidak boleh dihapus",
                colname
            ));
        }

        self.column.remove(index);

        for row in &mut self.row {
            if index < row.value.len() {
                row.value.remove(index);
            }
        }

        // Update primary_index jika perlu
        if let Some(p_idx) = self.primary_index {
            if p_idx > index {
                self.primary_index = Some(p_idx - 1);
            }
        }

        Ok(())
    }
    pub fn remove_row(&mut self, colname: &str, value: Value) -> Result<(), String> {
        let index = self
            .column
            .iter()
            .position(|c| c.colname == colname)
            .ok_or_else(|| format!("HAPUS_ROW: kolom `{}` tidak ditemukan", colname))?;

        self.row
            .retain(|row| row.value.get(index).map(|v| v != &value).unwrap_or(true));

        Ok(())
    }
    pub fn show_column_types(&self) {
        let mut pt = prettytable::Table::new();

        // Baris pertama: nama kolom
        let headers: Vec<prettytable::Cell> = self
            .column
            .iter()
            .map(|col| {
                if col.is_primary {
                    prettytable::Cell::new(&format!("*{}", col.colname))
                } else {
                    prettytable::Cell::new(&col.colname)
                }
            })
            .collect();
        pt.add_row(prettytable::Row::new(headers));

        // Baris kedua: tipe kolom
        let types: Vec<prettytable::Cell> = self
            .column
            .iter()
            .map(|col| {
                let col_type = match col.coltype {
                    ColumnType::Text => "Text",
                    ColumnType::Numb => "Numb",
                    ColumnType::Bool => "Bool",
                    ColumnType::Char => "Char",
                };
                prettytable::Cell::new(col_type)
            })
            .collect();
        pt.add_row(prettytable::Row::new(types));

        pt.printstd(); // cetak tabel dengan border, rapi, lebar dinamis
    }
    // Print tabel rapi
    pub fn show_data_table(&self) {
        use prettytable::format;

        let mut table = prettytable::Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        // --- HEADER ---
        let header_cells: Vec<prettytable::Cell> = self
            .column
            .iter()
            .map(|col| {
                let name = if col.is_primary {
                    format!("*{}", col.colname)
                } else {
                    col.colname.clone()
                };
                prettytable::Cell::new(&name)
                    .style_spec("c")
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::YELLOW))
            })
            .collect();

        table.add_row(prettytable::Row::new(header_cells));

        // --- ROWS ---
        for row in &self.row {
            // self.row: Vec<models::Row>
            let row_cells: Vec<prettytable::Cell> = row
                .value
                .iter()
                .map(|val| {
                    let s = match val {
                        Value::Text(v) => v.clone(),
                        Value::Char(c) => c.to_string(),
                        Value::Numb(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Empty => "-".into(),
                    };
                    prettytable::Cell::new(&s)
                })
                .collect();

            table.add_row(prettytable::Row::new(row_cells));
        }

        // --- PRINT TABLE ---
        table.printstd();
    }
}
