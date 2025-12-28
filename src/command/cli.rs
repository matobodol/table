use crate::table::handlers::service::DataTable;
// use crate::table::value::{ColumnType, Value};

pub fn run_command(table: &mut DataTable, args: Vec<String>) -> Result<(), String> {
    let cmd = &args[0];
    match cmd.as_str() {
        "add_column" => {
            /* parse args[1..] → add_column */
            Ok(())
        }
        "add_row" => {
            /* parse args[1..] → add_row */
            Ok(())
        }
        "set_primary" => {
            /* args[1] → set_primary */
            Ok(())
        }
        "set_value_where" => {
            /* args[1..] → set_value_where */
            Ok(())
        }
        "print" => {
            table.show_data_table();
            Ok(())
        }
        _ => Err(format!("Perintah '{}' tidak dikenali", cmd)),
    }
}
