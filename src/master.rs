use std::collections::HashMap;

use crate::table::handlers::service::DataTable;

pub struct DataBase {
    pub tables: HashMap<String, DataTable>,
    pub lot: Vec<String>, //lot: lis of tables (key tables)
    pub table_count: Option<usize>,
    pub selected: Option<String>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            lot: Vec::new(),
            table_count: None,
            selected: None,
        }
    }

    pub fn add_table(&mut self, name: &str) -> Result<(), String> {
        if self.tables.contains_key(name) {
            return Err(format!("Tabel `{}` sudah ada", name));
        }
        self.tables.insert(name.to_string(), DataTable::new());
        Ok(())
    }

    pub fn get_table(&mut self, name: &str) -> Option<&mut DataTable> {
        self.tables.get_mut(name)
    }

    pub fn remove_table(&mut self, name: &str) -> Result<(), String> {
        if self.tables.remove(name).is_some() {
            Ok(())
        } else {
            Err(format!("Tabel `{}` tidak ditemukan", name))
        }
    }
}
