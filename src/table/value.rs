use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColumnType {
    Text,
    Numb,
    Bool,
    Char,
}

use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Value {
    Text(String),
    Char(char),
    Numb(i64),
    Bool(bool),
    Empty,
}
