use crate::table::handlers::service::DataTable;
use crate::table::value::{ColumnType, Value};
use std::io::{self, Write};

pub fn run_menu(table: &mut DataTable) {
    loop {
        println!("\n=== Menu Tabel ===");
        println!("1. Tambah Kolom");
        println!("2. Tambah Row");
        println!("3. Set Primary");
        println!("4. Update Nilai");
        println!("5. Hapus Kolom / Row");
        println!("6. Tampilkan Tabel");
        println!("7. Tampilkan Tipe Kolom"); // <-- opsi baru
        println!("0. Keluar");

        let choice = read_input("Pilih menu: ");
        match choice.as_str() {
            "1" => add_column_interactive(table),
            "2" => add_row_interactive(table),
            "3" => set_primary_interactive(table),
            "4" => set_value_interactive(table),
            "5" => remove_interactive(table),
            "6" => table.show_data_table(),
            "7" => table.show_column_types(), // <-- panggil method baru
            "0" => break,
            _ => println!("Pilihan tidak valid"),
        }
    }
}

// --- Fungsi bantuan membaca input ---
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// --- Fungsi interaktif ---
fn add_column_interactive(table: &mut DataTable) {
    let col_name = read_input("Nama kolom: ");

    let col_type_str = read_input("Tipe kolom (Text, Numb, Bool, Char): ");

    let col_type = match col_type_str.to_lowercase().as_str() {
        "text" => ColumnType::Text,
        "numb" => ColumnType::Numb,
        "bool" => ColumnType::Bool,
        "char" => ColumnType::Char,
        _ => {
            println!("Tipe kolom tidak valid!");
            return;
        }
    };

    match table.add_column(vec![(col_name.as_str(), col_type)]) {
        Ok(_) => println!("Kolom berhasil ditambahkan."),
        Err(e) => println!("Error: {}", e),
    }
}

fn add_row_interactive(table: &mut DataTable) {
    if table.column.is_empty() {
        println!("Belum ada kolom, tambahkan kolom dulu!");
        return;
    }

    let mut values: Vec<Value> = Vec::new();

    for (i, col) in table.column.iter().enumerate() {
        loop {
            let input = read_input(&format!(
                "Nilai untuk kolom '{}' (type {:?}): ",
                col.colname, col.coltype
            ));

            match table._parse_input_to_value_type(i, &input) {
                Ok(val) => {
                    values.push(val);
                    break;
                }
                Err(e) => println!("Error: {}. Masukkan ulang.", e),
            }
        }
    }

    match table.add_row(values) {
        Ok(_) => println!("Row berhasil ditambahkan."),
        Err(e) => println!("Error: {}", e),
    }
}

fn set_primary_interactive(table: &mut DataTable) {
    let name = read_input("Nama kolom untuk primary: ");
    match table.set_primary(&name) {
        Ok(_) => println!("Primary column berhasil diatur"),
        Err(e) => println!("Error: {}", e),
    }
}

fn set_value_interactive(table: &mut DataTable) {
    let cond_col = read_input("Kolom kondisi: ");
    let cond_val_str = read_input("Nilai kondisi: ");
    let target_col = read_input("Kolom target: ");
    let new_val_str = read_input("Nilai baru: ");

    // Cari index kolom kondisi
    let cond_index = match table.column.iter().position(|c| c.colname == cond_col) {
        Some(i) => i,
        None => {
            println!("Kolom kondisi '{}' tidak ditemukan", cond_col);
            return;
        }
    };

    // Parse nilai kondisi sesuai tipe kolom
    let cond_value = match table._parse_input_to_value_type(cond_index, &cond_val_str) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    // Cari index kolom target
    let target_index = match table.column.iter().position(|c| c.colname == target_col) {
        Some(i) => i,
        None => {
            println!("Kolom target '{}' tidak ditemukan", target_col);
            return;
        }
    };

    // Parse nilai baru sesuai tipe kolom target
    let new_value = match table._parse_input_to_value_type(target_index, &new_val_str) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    // Set nilai
    match table.set_value_where(&cond_col, cond_value, &target_col, new_value) {
        Ok(_) => println!("Berhasil mengubah nilai."),
        Err(e) => println!("Error: {}", e),
    }
}

fn remove_interactive(table: &mut DataTable) {
    let pilih: String = read_input("Hapus kolom atau row? (kolom/row): ");
    match pilih.to_lowercase().as_str() {
        "kolom" => {
            let name: String = read_input("Nama kolom: ");
            match table.remove_column(&name) {
                Ok(_) => println!("Kolom dihapus"),
                Err(e) => println!("Error: {}", e),
            }
        }
        "row" => {
            let colname: String = read_input("Kolom kondisi: ");

            // Cari index kolom
            let col_index = match table.column.iter().position(|c| c.colname == colname) {
                Some(i) => i,
                None => {
                    println!("Error: kolom `{}` tidak ditemukan", colname);
                    return;
                }
            };

            // Baca input user
            let input_val: String = read_input("Nilai untuk hapus: ");

            // Parse input sesuai tipe kolom
            let val = match table._parse_input_to_value_type(col_index, &input_val) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };

            // Hapus row
            match table.remove_row(&colname, val) {
                Ok(_) => println!("Row dihapus"),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("Pilihan tidak valid"),
    }
}
