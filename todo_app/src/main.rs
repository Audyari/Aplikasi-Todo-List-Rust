use std::fs;
use std::io::{self, BufRead, Write};

#[derive(Debug)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

impl TodoItem {
    fn new(id: u32, title: &str) -> Self {
        // Menambahkan validasi untuk title tidak boleh kosong
        if title.trim().is_empty() {
            panic!("Title tidak boleh kosong");
        }
        
        TodoItem {
            id,
            title: title.to_string(),
            completed: false,
        }
    }

    fn mark_complete(&mut self) -> bool {
        let prev = self.completed;
        self.completed = true;
        !prev
    }

    fn update_title(&mut self, new_title: &str) -> bool {
        if new_title.is_empty() {
            return false;
        }
        self.title = new_title.to_string();
        true
    }

    // Method baru yang mungkin panic
    #[allow(dead_code)]
    fn get_by_index(&self, index: usize) -> char {
      
        self.title.chars().nth(index).expect("Indeks di luar jangkauan")
    }
}

// Struktur untuk mengelola banyak TodoItem
#[derive(Debug)]
struct TodoList {
    items: Vec<TodoItem>,
    next_id: u32,
}

impl TodoList {
    // Membuat TodoList baru
    fn new() -> Self {
        TodoList {
            items: Vec::new(),
            next_id: 1,
        }
    }

    // Menambahkan todo baru ke dalam list
    fn add_item(&mut self, title: &str) -> u32 {
        let id = self.next_id;
        let todo = TodoItem::new(id, title);
        self.items.push(todo);
        self.next_id += 1;
        id
    }

    // Menghapus todo berdasarkan ID
    fn remove_item(&mut self, id: u32) -> bool {
        if let Some(pos) = self.items.iter().position(|x| x.id == id) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    // Mendapatkan referensi ke todo berdasarkan ID
    fn get_item(&self, id: u32) -> Option<&TodoItem> {
        self.items.iter().find(|item| item.id == id)
    }

    // Mendapatkan mutable referensi ke todo berdasarkan ID
    fn get_item_mut(&mut self, id: u32) -> Option<&mut TodoItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    // Mendapatkan semua todo yang belum selesai
    fn get_active_items(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| !item.completed).collect()
    }

    // Mendapatkan semua todo yang sudah selesai
    fn get_completed_items(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| item.completed).collect()
    }

    // Menandai todo sebagai selesai
    fn mark_as_completed(&mut self, id: u32) -> bool {
        if let Some(item) = self.get_item_mut(id) {
            item.mark_complete();
            true
        } else {
            false
        }
    }

    // Mengupdate judul todo
    fn update_item_title(&mut self, id: u32, new_title: &str) -> bool {
        if let Some(item) = self.get_item_mut(id) {
            item.update_title(new_title)
        } else {
            false
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        use std::io::Write;
        
        let mut file = fs::File::create(filename)?;
        
        // Tulis header
        writeln!(&mut file, "id,completed,title")?;
        
        // Tulis setiap item
        for item in &self.items {
            // Pastikan tidak ada koma dalam title yang bisa mengacaukan format CSV
            let title = item.title.replace(",", " ");
            writeln!(
                &mut file,
                "{},{},{}",
                item.id,
                if item.completed { "true" } else { "false" },
                title
            )?;
        }
        
        // Pastikan semua data ditulis ke disk
        file.flush()?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        let file = fs::File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut list = TodoList::new();
        
        // Buat iterator dan lewati baris header
        let mut lines = reader.lines();
        if let Some(Ok(header)) = lines.next() {
            // Normalisasi header dengan menghapus whitespace
            let header = header.trim();
            if header != "id,completed,title" {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Format header tidak valid: '{}'", header)
                ));
            }
        }
        
        for (line_num, line_result) in lines.enumerate() {
            let line = line_result?;
            let line = line.trim();
            if line.is_empty() {
                continue; // Lewati baris kosong
            }
            
            let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            
            if parts.len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData, 
                    format!("Format baris {} tidak valid: '{:?}'", line_num + 2, parts)
                ));
            }
            
            let id = parts[0].parse::<u32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, 
                    format!("Gagal parse ID '{}': {}", parts[0], e)))?;
                
            let completed = match parts[1].to_lowercase().as_str() {
                "true" => true,
                "false" => false,
                _ => return Err(io::Error::new(io::ErrorKind::InvalidData, 
                    format!("Nilai completed tidak valid: '{}'", parts[1]))),
            };
                
            let title = parts[2].to_string();
            
            // Pastikan ID selalu yang terbesar
            if id >= list.next_id {
                list.next_id = id + 1;
            }
            
            list.items.push(TodoItem { id, title, completed });
        }
        
        Ok(list)
    }
}

/// Membersihkan layar dan mengembalikan kursor ke posisi awal
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

fn show_menu() {
    println!("\n=== Aplikasi Todo List ===");
    println!("1. Lihat Daftar Todo");
    println!("2. Tambah Todo Baru");
    println!("3. Tandai Selesai");
    println!("4. Update Judul Todo");
    println!("5. Hapus Todo");
    println!("6. Lihat Todo Aktif");
    println!("7. Lihat Todo Selesai");
    println!("8. Tampilkan Karakter dari Judul");
    println!("9. Simpan ke File");
    println!("10. Muat dari File");
    println!("0. Keluar");
    print!("\nPilihan Anda: ");
    io::stdout().flush().unwrap();
}

fn show_todo_list(todo_list: &TodoList) {
    if todo_list.items.is_empty() {
        println!("Tidak ada todo dalam daftar.");
        return;
    }

    println!("\n=== Daftar Todo ===");
    for item in &todo_list.items {
        let status = if item.completed { "✓" } else { " " };
        println!("[{}] {} {}", status, item.id, item.title);
    }
}

fn main() {
    // Coba muat dari file, jika gagal buat yang baru
    let mut todo_list = match TodoList::load_from_file("todos.csv") {
        Ok(list) => {
            println!("Berhasil memuat todo list dari file.");
            list
        },
        Err(e) => {
            println!("Tidak dapat memuat file: {}. Membuat todo list baru.", e);
            TodoList::new()
        }
    };
    
    let mut input = String::new();
    
    loop {
        clear_screen();
        show_menu();
        
        input.clear();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        
        match input.trim() {
            "1" => {
                show_todo_list(&todo_list);
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "2" => {
                print!("Masukkan judul todo: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).expect("Gagal membaca input");
                let title = input.trim();
                if !title.is_empty() {
                    let id = todo_list.add_item(title);
                    println!("Todo berhasil ditambahkan dengan ID: {}", id);
                } else {
                    println!("Judul tidak boleh kosong!");
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "3" => {
                show_todo_list(&todo_list);
                print!("\nMasukkan ID todo yang akan ditandai selesai: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).expect("Gagal membaca input");
                if let Ok(id) = input.trim().parse::<u32>() {
                    if todo_list.mark_as_completed(id) {
                        println!("Todo dengan ID {} telah ditandai selesai.", id);
                    } else {
                        println!("Gagal menandai todo. Pastikan ID benar dan todo belum selesai.");
                    }
                } else {
                    println!("ID tidak valid!");
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "4" => {
                show_todo_list(&todo_list);
                print!("\nMasukkan ID todo yang akan diupdate: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).expect("Gagal membaca input");
                
                if let Ok(id) = input.trim().parse::<u32>() {
                    print!("Masukkan judul baru: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Gagal membaca input");
                    let new_title = input.trim();
                    
                    if !new_title.is_empty() {
                        if todo_list.update_item_title(id, new_title) {
                            println!("Judul todo berhasil diupdate.");
                        } else {
                            println!("Gagal mengupdate todo. Pastikan ID benar.");
                        }
                    } else {
                        println!("Judul tidak boleh kosong!");
                    }
                } else {
                    println!("ID tidak valid!");
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "5" => {
                show_todo_list(&todo_list);
                print!("\nMasukkan ID todo yang akan dihapus: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).expect("Gagal membaca input");
                
                if let Ok(id) = input.trim().parse::<u32>() {
                    if todo_list.remove_item(id) {
                        println!("Todo berhasil dihapus.");
                    } else {
                        println!("Gagal menghapus todo. Pastikan ID benar.");
                    }
                } else {
                    println!("ID tidak valid!");
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "6" => {
                let active_items = todo_list.get_active_items();
                if active_items.is_empty() {
                    println!("\nTidak ada todo aktif.");
                } else {
                    println!("\n=== Todo Aktif ===");
                    for item in active_items {
                        println!("[ ] {} {}", item.id, item.title);
                    }
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "7" => {
                let completed_items = todo_list.get_completed_items();
                if completed_items.is_empty() {
                    println!("\nTidak ada todo yang selesai.");
                } else {
                    println!("\n=== Todo Selesai ===");
                    for item in completed_items {
                        println!("[✓] {} {}", item.id, item.title);
                    }
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "8" => {
                show_todo_list(&todo_list);
                print!("\nMasukkan ID todo: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).expect("Gagal membaca input");
                
                if let Ok(id) = input.trim().parse::<u32>() {
                    if let Some(item) = todo_list.get_item(id) {
                        print!("Masukkan indeks karakter yang ingin ditampilkan (0-{}): ", item.title.chars().count().saturating_sub(1));
                        io::stdout().flush().unwrap();
                        input.clear();
                        io::stdin().read_line(&mut input).expect("Gagal membaca input");
                        
                        if let Ok(index) = input.trim().parse::<usize>() {
                            match item.title.chars().nth(index) {
                                Some(c) => println!("Karakter pada indeks {}: '{}'", index, c),
                                None => println!("Indeks di luar jangkauan. Panjang judul: {} karakter", item.title.chars().count()),
                            }
                        } else {
                            println!("Indeks tidak valid!");
                        }
                    } else {
                        println!("Todo dengan ID {} tidak ditemukan.", id);
                    }
                } else {
                    println!("ID tidak valid!");
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "9" => {
                if let Err(e) = todo_list.save_to_file("todos.csv") {
                    println!("Gagal menyimpan ke file: {}", e);
                } else {
                    println!("Todo list berhasil disimpan ke todos.csv");
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "10" => {
                match TodoList::load_from_file("todos.csv") {
                    Ok(loaded_list) => {
                        todo_list = loaded_list;
                        println!("Todo list berhasil dimuat dari todos.csv");
                    },
                    Err(e) => println!("Gagal memuat dari file: {}", e),
                }
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "0" => {
                println!("\nTerima kasih telah menggunakan Aplikasi Todo List!");
                break;
            },
            _ => {
                println!("Pilihan tidak valid!");
                println!("\nTekan Enter untuk melanjutkan...");
                io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // 1. Test dengan assert_eq!
    #[test]
    fn test_todo_creation() {
        let todo = TodoItem::new(1, "Belajar Rust");
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Belajar Rust");
        assert_eq!(todo.completed, false);
    }

    // 2. Test dengan assert!
    #[test]
    fn test_mark_complete() {
        let mut todo = TodoItem::new(1, "Belajar Rust");
        assert!(!todo.completed, "Awalnya todo harus belum selesai");
        
        let changed = todo.mark_complete();
        assert!(todo.completed, "Setelah mark_complete, todo harus selesai");
        assert!(changed, "mark_complete harus mengembalikan true jika status berubah");
        
        // Memanggil lagi seharusnya mengembalikan false karena status tidak berubah
        let changed_again = todo.mark_complete();
        assert!(!changed_again, "mark_complete harus mengembalikan false jika status tidak berubah");
    }

    // 3. Test dengan assert_ne!
    #[test]
    fn test_update_title() {
        let mut todo = TodoItem::new(1, "Belajar");
        let old_title = todo.title.clone();
        
        let updated = todo.update_title("Belajar Rust");
        assert!(updated, "Update title harus berhasil");
        assert_ne!(todo.title, old_title, "Judul harus berubah");
        assert_ne!(todo.title, "", "Judul tidak boleh kosong");
    }

    // 4. Test dengan should_panic
    #[test]
    #[should_panic(expected = "Title tidak boleh kosong")]
    fn test_empty_title_panic() {
        // Seharusnya panic karena judul kosong
        let _ = TodoItem::new(1, "");
    }

    // 5. Test yang di-ignore
    #[test]
    #[ignore = "Alasan mengapa test ini di-ignore"]
    fn test_ignored() {
        panic!("Test ini tidak akan pernah dijalankan karena di-ignore");
    }

    // 6. Test yang mengembalikan Result
    #[test]
    fn test_with_result() -> Result<(), String> {
        let todo = TodoItem::new(1, "Test Result");
        
        if todo.id != 1 {
            return Err(String::from("ID tidak sesuai"));
        }
        
        if todo.title != "Test Result" {
            return Err(String::from("Judul tidak sesuai"));
        }
        
        Ok(())
    }

    // 7. Test untuk operasi TodoList
    #[test]
    fn test_todo_list_operations() {
        let mut list = TodoList::new();
        
        // Test add_item
        let id1 = list.add_item("Belajar");
        let id2 = list.add_item("Makan");
        
        // Test get_item
        assert!(list.get_item(id1).is_some(), "Item dengan id1 harus ada");
        assert!(list.get_item(id2).is_some(), "Item dengan id2 harus ada");
        assert!(list.get_item(999).is_none(), "Item dengan ID tidak ada harus mengembalikan None");
        
        // Test mark_as_completed
        assert!(list.mark_as_completed(id1), "Mark as completed harus berhasil");
        assert!(!list.mark_as_completed(999), "Mark as completed dengan ID tidak valid harus gagal");
        
        // Test get_active_items dan get_completed_items
        assert_eq!(list.get_active_items().len(), 1, "Harus ada 1 item aktif");
        assert_eq!(list.get_completed_items().len(), 1, "Harus ada 1 item selesai");
        
        // Test remove_item
        assert!(list.remove_item(id1), "Hapus item harus berhasil");
        assert!(!list.remove_item(999), "Hapus item dengan ID tidak valid harus gagal");
        assert!(list.get_item(id1).is_none(), "Item yang dihapus tidak boleh ada");
    }

    // 8. Test untuk error handling dengan Result dan unwrap_err
    #[test]
    fn test_error_handling() {
        let result: Result<(), &str> = Err("Terjadi error");
        
        // Menggunakan unwrap_err untuk memastikan error yang diharapkan
        let error_msg = result.unwrap_err();
        assert_eq!(error_msg, "Terjadi error");
    }

    // 9. Test untuk get_by_index
    #[test]
    fn test_get_by_index() {
        let todo = TodoItem::new(1, "Rust");
        assert_eq!(todo.get_by_index(0), 'R');
        assert_eq!(todo.get_by_index(1), 'u');
        assert_eq!(todo.get_by_index(2), 's');
        assert_eq!(todo.get_by_index(3), 't');
    }

    // 10. Test panic untuk get_by_index dengan indeks di luar jangkauan
    #[test]
    #[should_panic(expected = "Indeks di luar jangkauan")]
    fn test_get_by_index_out_of_bounds() {
        let todo = TodoItem::new(1, "Rust");
        let _ = todo.get_by_index(10); // Seharusnya panic
    }

    // 11. Test untuk save_to_file dan load_from_file
    #[test]
    fn test_save_and_load_file() {
        use std::fs;
        
        // Buat file sementara untuk testing
        let temp_file = "test_todos.csv";
        
        // Pastikan file tidak ada di awal test
        let _ = fs::remove_file(temp_file);
        
        // Buat TodoList baru dan tambahkan beberapa item
        let mut todo_list = TodoList::new();
        let id1 = todo_list.add_item("Belajar Rust");
        let id2 = todo_list.add_item("Makan siang");
        todo_list.mark_as_completed(id1);
        
        // Simpan ke file
        todo_list.save_to_file(temp_file).expect("Gagal menyimpan ke file");
        
        // Pastikan file berhasil dibuat
        assert!(fs::metadata(temp_file).is_ok(), "File harus berhasil dibuat");
        
        // Muat dari file
        let loaded_list = TodoList::load_from_file(temp_file).expect("Gagal memuat dari file");
        
        // Verifikasi data yang dimuat
        assert_eq!(loaded_list.items.len(), 2, "Jumlah item harus sama");
        
        // Verifikasi item pertama
        let item1 = loaded_list.get_item(id1).expect("Item 1 harus ada");
        assert_eq!(item1.title, "Belajar Rust");
        assert!(item1.completed, "Item 1 harus selesai");
        
        // Verifikasi item kedua
        let item2 = loaded_list.get_item(id2).expect("Item 2 harus ada");
        assert_eq!(item2.title, "Makan siang");
        assert!(!item2.completed, "Item 2 belum selesai");
        
        // Bersihkan file sementara
        fs::remove_file(temp_file).expect("Gagal menghapus file sementara");
    }

    // 12. Test untuk load_from_file dengan file yang tidak ada
    #[test]
    fn test_load_nonexistent_file() {
        let non_existent_file = "file_tidak_ada_12345.csv"; // Nama file yang pasti tidak ada
        let result = TodoList::load_from_file(non_existent_file);
        
        // Verifikasi bahwa mengembalikan error
        assert!(result.is_err(), "Harus mengembalikan error untuk file yang tidak ada");
    }
}
