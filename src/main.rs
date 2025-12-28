mod command;
// mod master;
mod menu;
mod table;

use std::env;
use table::handlers::service::DataTable;

// Lokasi file penyimpanan tabel
const TABLE_FILE: &str = "/data/data/com.termux/files/home/.mytabel.json";

fn main() {
    // --- Load tabel jika file ada ---
    let mut table = match DataTable::load(TABLE_FILE) {
        Ok(t) => t,
        Err(_) => DataTable::new(), // jika gagal load, buat tabel baru
    };

    // Ambil argumen
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        // --- Mode Menu Interaktif ---
        menu::interactive::run_menu(&mut table);

        // Simpan otomatis setelah keluar dari menu
        if let Err(e) = table.save(TABLE_FILE) {
            eprintln!("Gagal menyimpan tabel: {}", e);
        }
    } else {
        // --- Mode Command-Line ---
        if let Err(e) = command::cli::run_command(&mut table, args) {
            eprintln!("Error: {}", e);
        }

        // Simpan tabel setelah menjalankan command
        if let Err(e) = table.save(TABLE_FILE) {
            eprintln!("Gagal menyimpan tabel: {}", e);
        }
    }
}

// mod command;
// mod menu;
// mod table;
//
// use std::env;
//
// use table::handlers::service::DataTable;
//
// fn main() {
//     // Buat instance tabel
//     let mut table = DataTable::new();
//
//     // Ambil argumen
//     let args: Vec<String> = env::args().skip(1).collect(); // skip executable name
//
//     if args.is_empty() {
//         // --- Mode Menu Interaktif ---
//         menu::interactive::run_menu(&mut table);
//     } else {
//         // --- Mode Command-Line ---
//         if let Err(e) = command::cli::run_command(&mut table, args) {
//             eprintln!("Error: {}", e);
//         }
//     }
// }
