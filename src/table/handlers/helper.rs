use crate::table::{
    handlers::service::DataTable,
    value::{ColumnType, Value},
};
use std::collections::HashSet;
use std::fs;

impl DataTable {
    pub fn save(&self, path: &str) -> Result<(), String> {
        let json =
            serde_json::to_string_pretty(self).map_err(|e| format!("Gagal serialisasi: {}", e))?;
        fs::write(path, json).map_err(|e| format!("Gagal tulis file: {}", e))
    }

    pub fn load(path: &str) -> Result<Self, String> {
        let data = fs::read_to_string(path).map_err(|e| format!("Gagal baca file: {}", e))?;
        let table: DataTable =
            serde_json::from_str(&data).map_err(|e| format!("Gagal parse JSON: {}", e))?;
        Ok(table)
    }
    pub fn _parse_input_to_value_type(
        &self,
        col_index: usize,
        input: &str,
    ) -> Result<Value, String> {
        let col_type = &self.column[col_index].coltype;
        match col_type {
            ColumnType::Text => Ok(Value::Text(input.into())),
            ColumnType::Char => {
                if input.len() == 1 {
                    Ok(Value::Char(input.chars().next().unwrap()))
                } else {
                    Err("Char harus 1 karakter".into())
                }
            }
            ColumnType::Numb => input
                .parse::<i64>()
                .map(Value::Numb)
                .map_err(|_| format!("Numb harus angka: '{}'", input)),
            ColumnType::Bool => match input.to_lowercase().as_str() {
                "true" | "1" => Ok(Value::Bool(true)),
                "false" | "0" => Ok(Value::Bool(false)),
                _ => Err("Bool harus true/false atau 1/0".into()),
            },
        }
    }

    // Validasi kandidat primary
    pub fn _validate_primary_candidate(&self, col_index: usize) -> Result<(), String> {
        // 1. Tidak boleh Empty
        for row in &self.row {
            if matches!(row.value[col_index], Value::Empty) {
                return Err("PRIMARY: ada nilai Empty".into());
            }
        }

        // 2. Harus unik
        let mut set: HashSet<Value> = HashSet::new();
        for row in &self.row {
            if !set.insert(row.value[col_index].clone()) {
                return Err("PRIMARY: nilai duplikat".into());
            }
        }

        // 3. Tipe harus cocok
        for row in &self.row {
            if !Self::_validate_type_column_and_row(
                &self.column[col_index].coltype,
                &row.value[col_index],
            ) {
                return Err("PRIMARY: tipe tidak cocok".into());
            }
        }

        Ok(())
    }
    // Validasi tipe untuk Value vs ColumnType
    pub fn _validate_type_column_and_row(col_type: &ColumnType, value: &Value) -> bool {
        match value {
            Value::Empty => true, // pengecualian: Empty boleh ke mana saja
            Value::Text(_) => matches!(col_type, ColumnType::Text),
            Value::Char(_) => matches!(col_type, ColumnType::Char),
            Value::Numb(_) => matches!(col_type, ColumnType::Numb),
            Value::Bool(_) => matches!(col_type, ColumnType::Bool),
        }
    }
    pub fn _increment(&mut self) -> i64 {
        self._increment += 1;
        self._increment
    }
    pub fn _decrement(&mut self) -> i64 {
        self._increment -= 1;
        self._increment
    }
    // Ambil primary
    pub fn _get_column_is_primary_active(&self) -> Option<&str> {
        self.column
            .iter()
            .find_map(|c| c.is_primary.then(|| c.colname.as_str()))
    }

    // Ambil kolom auto_increment
    pub fn _get_column_is_increment_active(&self) -> Option<&str> {
        self.column
            .iter()
            .find_map(|c| c._is_auto_increment.then(|| c.colname.as_str()))
    }
}
