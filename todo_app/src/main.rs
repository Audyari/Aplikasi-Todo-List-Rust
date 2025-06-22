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
      fn get_by_index(&self, index: usize) -> char {
        self.title.chars().nth(index).expect("Indeks di luar jangkauan")
    }
}

// Struktur untuk mengelola banyak TodoItem
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
        let mut file = fs::File::create(filename)?;
        writeln!(file, "id,completed,title")?;  // Header
        
        for item in &self.items {
            writeln!(file, "{},{},{}", 
                item.id, 
                if item.completed { "true" } else { "false" }, 
                item.title.replace(",", "\\,")  // Handle koma di judul
            )?;
        }
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        let file = fs::File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut list = TodoList::new();
        
        // Lewati header
        let lines = reader.lines().skip(1);
        
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 3 {
                if let (Ok(id), completed, title) = (
                    parts[0].parse::<u32>(),
                    parts[1] == "true",
                    parts[2..].join(",").replace("\\,", ",")
                ) {
                    let mut item = TodoItem::new(id, &title);
                    if completed {
                        item.mark_complete();
                    }
                    list.items.push(item);
                    list.next_id = list.next_id.max(id + 1);
                }
            }
        }
        Ok(list)
    }
}

fn main() {
    // Membuat todo baru
    let mut todo = TodoItem::new(1, "Belajar Rust");
    println!("Todo awal: {} - {} (Selesai: {})", todo.id, todo.title, todo.completed);
    
    // Menggunakan method update_title
    let update_result = todo.update_title("Belajar Rust Lanjutan");
    println!("Update judul: {}", if update_result { "Berhasil" } else { "Gagal" });
    
    // Menggunakan method get_by_index
    match todo.title.chars().count() > 0 {
        true => {
            let first_char = todo.get_by_index(0);
            println!("Karakter pertama judul: {}", first_char);
        }
        false => println!("Judul kosong"),
    }
    
    // Mencoba update dengan string kosong
    let update_kosong = todo.update_title("");
    println!("Update kosong: {}", if update_kosong { "Berhasil" } else { "Gagal (seharusnya)" });
    
    // Menandai todo sebagai selesai
    let status = todo.mark_complete();
    println!("Setelah mark_complete: Selesai: {} (Status perubahan: {})", todo.completed, status);

    // Contoh penggunaan TodoList
    let mut todo_list = TodoList::new();
    
    // Menambahkan beberapa todo
    let id1 = todo_list.add_item("Belajar Rust");
    let id2 = todo_list.add_item("Buat aplikasi Todo");
    
    // Mendapatkan dan menampilkan item menggunakan get_item
    if let Some(item) = todo_list.get_item(id1) {
        println!("Item dengan ID {}: {}", id1, item.title);
    }
    
    // Mengupdate judul menggunakan update_item_title
    todo_list.update_item_title(id2, "Buat aplikasi Todo List");
    
    // Menandai todo sebagai selesai SEBELUM menyimpan ke file
    todo_list.mark_as_completed(id1);
    
    // Menyimpan ke file
    if let Err(e) = todo_list.save_to_file("todos.csv") {
        eprintln!("Gagal menyimpan ke file: {}", e);
    } else {
        println!("Berhasil menyimpan ke todos.csv");
    }
    
    // Memuat dari file
    match TodoList::load_from_file("todos.csv") {
        Ok(loaded_list) => {
            println!("\n=== Data yang dimuat dari file ===");
            println!("Total todo: {}", loaded_list.items.len());
            println!("Total aktif: {}", loaded_list.get_active_items().len());
            println!("Total selesai: {}", loaded_list.get_completed_items().len());
            
            // Tampilkan semua item yang dimuat
            for item in &loaded_list.items {
                println!("- {}: {} ({})", 
                    item.id, 
                    item.title, 
                    if item.completed { "Selesai" } else { "Belum selesai" }
                );
            }
            println!("==============================\n");
        }
        Err(e) => eprintln!("Gagal memuat dari file: {}", e),
    }
    
    // Menampilkan jumlah todo aktif
    println!("Total todo aktif: {}", todo_list.get_active_items().len());
    
    // Menampilkan jumlah todo yang sudah selesai
    println!("Total todo selesai: {}", todo_list.get_completed_items().len());
    
    // Menghapus todo
    todo_list.remove_item(id2);
    
    // Menampilkan jumlah todo aktif setelah penghapusan
    println!("Total todo aktif setelah penghapusan: {}", todo_list.get_active_items().len());
}

#[cfg(test)]
mod tests {
    
    use super::*;

    // case 1 Testing assert_eq! for TodoItem::new()
    #[test]
    fn test_todo_creation() {
        let todo = TodoItem::new(1, "Belajar Rust");
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Belajar Rust");
        assert_eq!(todo.completed, false);
    }

    // 2. Test menggunakan assert! untuk TodoItem::mark_complete()
    #[test]
    fn test_mark_complete_assert() {
        let mut todo = TodoItem::new(1, "Belajar Rust");
           
        // assert! untuk mengecek kondisi boolean
        assert!(!todo.completed, "Status awal harus false");
           
        let status = todo.mark_complete();
        assert!(todo.completed, "Status harus berubah menjadi true");
        assert!(status, "Harus mengembalikan true karena status berubah");
           
        // Test idempotent
        let status_kedua = todo.mark_complete();
        assert!(!status_kedua, "Harus mengembalikan false karena status tidak berubah");
    }

    // case 3 Testing assert_eq! for TodoItem::mark_complete()
    #[test]
    fn test_mark_complete() {
        let mut todo = TodoItem::new(1, "Belajar Rust");
        let old_title = todo.title.clone();
        
        let result = todo.update_title("Belajar Rust Lanjutan");
        assert!(result, "Update harus berhasil");
        assert_ne!(todo.title, old_title, "Judul harus berubah");
        
        // Test update dengan string kosong
        let result = todo.update_title("");
        assert!(!result, "Update dengan string kosong harus gagal");
        assert_ne!(todo.title, "", "Judul tidak boleh kosong");
    }

    // 4.1. Test panic dengan pesan spesifik for TodoItem::new()
    #[test]
    #[should_panic(expected = "Title tidak boleh kosong")]
    fn test_empty_title_panic() {
        let _ = TodoItem::new(1, ""); // Seharusnya panic
    }

    // 2. Test panic dengan method yang memanggil expect
    #[test]
    #[should_panic(expected = "Indeks di luar jangkauan")]
    fn test_invalid_index_access() {
        let todo = TodoItem::new(1, "Test");
        let _ = todo.get_by_index(10); // Seharusnya panic karena indeks melebihi panjang string
    }

    // 4.2. Test panic dengan Result for TodoItem::new()
    #[test]
    fn test_result_panic() -> Result<(), String> {
        let todo = TodoItem::new(1, "Test");
         
        // Contoh menggunakan Result
        if todo.id != 1 {
            return Err(String::from("ID tidak sesuai"));
        }
         
        // Test panic dengan unwrap pada Result
        let result: Result<u32, &str> = Ok(42);
        assert_eq!(result.unwrap(), 42);
         
        Ok(())
    }

    // 4.3. Test panic dengan unwrap_err for TodoItem::new()
    #[test]
    fn test_unwrap_err() {
        let result: Result<u32, &str> = Err("Error message");
        assert_eq!(result.unwrap_err(), "Error message");
    }

    // case 5. Test TodoList operations
    #[test]
    fn test_todo_list_operations() {
        let mut list = TodoList::new();
        
        // Test add_item
        let id1 = list.add_item("Test Todo 1");
        assert_eq!(list.items.len(), 1);
        assert_eq!(list.get_item(id1).unwrap().title, "Test Todo 1");
        
        // Test mark_as_completed
        assert!(list.mark_as_completed(id1));
        assert!(list.get_item(id1).unwrap().completed);
        
        // Test update_item_title
        assert!(list.update_item_title(id1, "Updated Title"));
        assert_eq!(list.get_item(id1).unwrap().title, "Updated Title");
        
        // Test remove_item
        assert!(list.remove_item(id1));
        assert!(list.get_item(id1).is_none());
        
        // Test non-existent item
        assert!(!list.mark_as_completed(999));
        assert!(!list.update_item_title(999, "Test"));
    }
}
