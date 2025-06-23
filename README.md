# Aplikasi Todo List dengan Rust - Focus on Unit Testing

## Overview Proyek
Membangun aplikasi Todo List sederhana menggunakan Rust dengan fokus utama pada pembelajaran **Unit Testing** yang komprehensif. Setiap fitur akan dibangun dengan Test-Driven Development (TDD) approach.

## Task 1: Setup Proyek dan Testing Environment

### 1.1 Inisialisasi Proyek
- [ ] Jalankan `cargo new todo_app`
- [ ] Masuk ke direktori proyek `cd todo_app`
- [ ] Explore struktur proyek (src/lib.rs vs src/main.rs)
- [ ] Test build awal dengan `cargo test`

### 1.2 Understand Basic Testing Structure
- [ ] Baca dan pahami default test di `src/lib.rs`
- [ ] Jalankan `cargo test` dan lihat output
- [ ] Modifikasi test untuk memahami assert macros
- [ ] Coba `cargo test -- --nocapture` untuk melihat println! output


### 2.1 Eksplorasi Assert Macros

 - `assert_eq!()` - untuk comparison  
 - `assert!()` - untuk boolean expression  
 - `assert_ne!()` - untuk negation comparison   
 - `# [should_panic]` - untuk expected failures
 - `#[ignore]` - untuk skipping tests bila tidak ingin dijalankan
 - `Test Result` - untuk testing error handling

## Task 2: Setup Documentation

### 2.1 Setup Documentation
- [ ] Jalankan `cargo doc --open`
- [ ] Pelajari `#[doc]` attribute
- [ ] Setup `docs/` directory untuk documentation
- [ ] Understand documentation vs source code differences
- [ ] Jalankan specific documentation dengan `cargo doc -- --nocapture`

### 2.2 Test Integration Test 
- [ ] Test Integration Test

### 2.3 Test Module
- [ ] Test Module

