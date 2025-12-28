use crate::table::value::{ColumnType, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnTable {
    pub colname: String,
    pub coltype: ColumnType,
    pub is_primary: bool,
    pub _is_auto_increment: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowTable {
    pub value: Vec<Value>,
}
