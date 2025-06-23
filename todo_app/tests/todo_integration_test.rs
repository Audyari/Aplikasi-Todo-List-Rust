// tests/todo_integration_test.rs
use todo_lib::TodoList;
use std::fs;

#[test]
fn test_multiple_items() {
    let mut list = TodoList::new();
    list.add_item("Satu");
    list.add_item("Dua");
    list.add_item("Tiga");
    
    assert_eq!(list.items.len(), 3);
    assert_eq!(list.items[0].title, "Satu");
    assert_eq!(list.items[1].title, "Dua");
    assert_eq!(list.items[2].title, "Tiga");
}


#[test]
fn test_remove_nonexistent() {
    let mut list = TodoList::new();
    list.add_item("Test");
    let initial_len = list.items.len();
    
    // Coba hapus index yang tidak ada
    list.remove_item(999);
    assert_eq!(list.items.len(), initial_len); // Panjang harus tetap sama
}

#[test]
fn test_mark_complete_invalid_index() {
    let mut list = TodoList::new();
    list.add_item("Test");
    
    // Coba tandai index yang tidak ada
    let result = list.mark_as_completed(999);
    assert!(!result, "Harus mengembalikan false untuk index tidak valid");
}

#[test]
fn test_load_nonexistent_file() {
    let filename = "file_tidak_ada_12345.csv";
    // Pastikan file benar-benar tidak ada
    let _ = std::fs::remove_file(filename);
    
    let result = TodoList::load_from_file(filename);
    assert!(result.is_err(), "Harus mengembalikan error untuk file yang tidak ada");
}

#[test]
fn test_load_invalid_file() {
    let filename = "invalid.csv";
    // Buat file dengan format tidak valid
    std::fs::write(filename, "bukan format csv yang valid").unwrap();
    
    let result = TodoList::load_from_file(filename);
    assert!(result.is_err(), "Harus mengembalikan error untuk file tidak valid");
    
    // Bersihkan
    let _ = std::fs::remove_file(filename);
}

#[test]
fn test_add_remove_items() {
    let mut todo_list = TodoList::new();
    
    // Test menambah item
    todo_list.add_item("Belajar Rust");
    assert_eq!(todo_list.items.len(), 1);
    assert_eq!(todo_list.items[0].title, "Belajar Rust");
    
    // Test menghapus item
    todo_list.remove_item(0);
    assert!(todo_list.items.is_empty());
}

#[test]
fn test_mark_complete() {
    let mut todo_list = TodoList::new();
    todo_list.add_item("Task 1");
    
    // Pastikan awalnya belum selesai
    assert!(!todo_list.items[0].completed);
    
    // Tandai selesai
    todo_list.mark_as_completed(0);
    assert!(todo_list.items[0].completed);
    
    // Test assert_ne!
    assert_ne!(todo_list.items[0].completed, false);
}

// Test dengan #[should_panic]
#[test]
#[should_panic(expected = "Title tidak boleh kosong")]
fn test_empty_title() {
    let mut todo_list = TodoList::new();
    todo_list.add_item("");  // Seharusnya panic
}

// Test yang di-ignore
#[test]
#[ignore = "Ini adalah test yang sengaja di-skip"]
fn test_ignored() {
    panic!("Test ini tidak akan pernah dijalankan karena di-ignore");
}

// Test yang mengembalikan Result
#[test]
fn test_with_result() -> Result<(), String> {
    let mut todo_list = TodoList::new();
    todo_list.add_item("Test Result");
    
    if todo_list.items.is_empty() {
        return Err(String::from("Gagal menambahkan item"));
    }
    
    assert_eq!(todo_list.items[0].title, "Test Result");
    Ok(())
}

#[test]
fn test_save_and_load() {
    let test_file = "test_todos.csv";
    
    // Pastikan file test bersih
    let _ = fs::remove_file(test_file);
    
    // Buat dan simpan todo list
    let mut todo_list = TodoList::new();
    todo_list.add_item("Belajar Integration Test");
    
    // Test save
    let save_result = todo_list.save_to_file(test_file);
    assert!(save_result.is_ok(), "Gagal menyimpan ke file");
    
    // Pastikan file dibuat
    assert!(std::path::Path::new(test_file).exists());
    
    // Test load
    let load_result = TodoList::load_from_file(test_file);
    assert!(load_result.is_ok(), "Gagal memuat dari file");
    
    let loaded_list = load_result.unwrap();
    
    // Verifikasi
    assert_eq!(loaded_list.items.len(), 1);
    assert_eq!(loaded_list.items[0].title, "Belajar Integration Test");
    assert!(!loaded_list.items[0].completed);
    
    // Bersihkan
    let remove_result = fs::remove_file(test_file);
    assert!(remove_result.is_ok(), "Gagal menghapus file test");
}
