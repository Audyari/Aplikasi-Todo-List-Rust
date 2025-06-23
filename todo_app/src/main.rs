// src/main.rs

use todo_lib::{TodoList};
use std::io::Write;
use std::io;

fn main() {
    let mut todo_list = TodoList::new();
    
    // Kode utama aplikasi CLI
    loop {
        println!("\n=== Todo List ===");
        println!("1. Lihat Daftar");
        println!("2. Tambah Todo");
        println!("3. Tandai Selesai");
        println!("4. Hapus Todo");
        println!("5. Simpan ke File");
        println!("6. Muat dari File");
        println!("0. Keluar");
        
        print!("\nPilih menu: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => show_todos(&todo_list),
            "2" => add_todo(&mut todo_list),
            "3" => mark_completed(&mut todo_list),
            "4" => remove_todo(&mut todo_list),
            "5" => save_to_file(&todo_list),
            "6" => todo_list = load_from_file(),
            "0" => break,
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

// Fungsi-fungsi helper untuk CLI
fn show_todos(list: &TodoList) {
    println!("\n=== Daftar Todo ===");
    for (i, item) in list.items.iter().enumerate() {
        println!("{}. [{}] {}", i + 1, if item.completed { "x" } else { " " }, item.title);
    }
}

fn add_todo(list: &mut TodoList) {
    println!("\nTambah Todo Baru");
    print!("Judul: ");
    io::stdout().flush().unwrap();
    
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    
    if !title.trim().is_empty() {
        list.add_item(title.trim());
        println!("Todo berhasil ditambahkan!");
    } else {
        println!("Judul tidak boleh kosong!");
    }
}

fn mark_completed(list: &mut TodoList) {
    show_todos(list);
    if list.items.is_empty() {
        return;
    }
    
    print!("\nNomor todo yang selesai: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= list.items.len() {
            list.mark_as_completed(index - 1);
            println!("Todo ditandai selesai!");
        } else {
            println!("Nomor tidak valid!");
        }
    } else {
        println!("Input tidak valid!");
    }
}

fn remove_todo(list: &mut TodoList) {
    show_todos(list);
    if list.items.is_empty() {
        return;
    }
    
    print!("\nNomor todo yang dihapus: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= list.items.len() {
            list.remove_item(index - 1);
            println!("Todo dihapus!");
        } else {
            println!("Nomor tidak valid!");
        }
    } else {
        println!("Input tidak valid!");
    }
}

fn save_to_file(list: &TodoList) {
    print!("\nNama file untuk disimpan (default: todos.csv): ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    
    // Gunakan default jika input kosong, hapus whitespace/newline
    let mut filename = filename.trim().to_string();
    if filename.is_empty() {
        filename = "todos.csv".to_string();
    }
    
    // Pastikan ekstensi .csv
    if !filename.ends_with(".csv") {
        filename.push_str(".csv");
    }
    
    match list.save_to_file(&filename) {
        Ok(_) => println!("Berhasil disimpan ke {}", filename),
        Err(e) => println!("Gagal menyimpan: {}", e),
    }
}

fn load_from_file() -> TodoList {
    print!("\nNama file yang akan dimuat (default: todos.csv): ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    
    // Gunakan default jika input kosong, hapus whitespace/newline
    let mut filename = filename.trim().to_string();
    if filename.is_empty() {
        filename = "todos.csv".to_string();
    }
    
    // Tambahkan .csv jika belum ada
    if !filename.ends_with(".csv") {
        filename.push_str(".csv");
    }
    
    match TodoList::load_from_file(&filename) {
        Ok(list) => {
            println!("Berhasil memuat dari {}", filename);
            list
        },
        Err(e) => {
            println!("Gagal memuat: {}", e);
            TodoList::new()
        }
    }
}