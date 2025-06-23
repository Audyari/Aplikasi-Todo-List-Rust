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
 - `  Test Result` - untuk testing error handling

## Task 2: Setup Documentation

### 2.1 Setup Documentation
- [ ] Jalankan `cargo doc --open`
- [ ] Pelajari `#[doc]` attribute
- [ ] Setup `docs/` directory untuk documentation
- [ ] Understand documentation vs source code differences
- [ ] Jalankan specific documentation dengan `cargo doc -- --nocapture`

### 2.2 Test Organization
- [ ] Pelajari `#[cfg(test)]` attribute
- [ ] Setup `tests/` directory untuk integration tests
- [ ] Understand unit test vs integration test differences
- [ ] Jalankan specific tests dengan `cargo test test_name`

### 2.3 Test Attributes dan Configuration
- [ ] Eksplorasi `#[test]` attribute
- [ ] Implementasi `#[ignore]` untuk skipping tests
- [ ] Test dengan `#[should_panic(expected = "message")]`
- [ ] Setup test timeout dengan custom attributes
- [ ] Parallel vs sequential test execution

## Task 3: TDD - Todo Item Structure

### 3.1 Write Tests First - Todo Item
- [ ] Buat file `src/todo_item.rs`
- [ ] Tulis test untuk `TodoItem::new()` **SEBELUM** implementasi
- [ ] Test properties: id, title, description, completed, created_at
- [ ] Jalankan test (harus fail) - Red phase
- [ ] Implementasi minimal untuk pass test - Green phase
- [ ] Refactor code - Refactor phase

### 3.2 Test TodoItem Methods
- [ ] Test `mark_completed()` method:
  - Test state change dari false ke true
  - Test idempotency (memanggil berkali-kali)
  - Test return value
- [ ] Test `mark_incomplete()` method
- [ ] Test `update_title()` method:
  - Test title change
  - Test empty title handling
  - Test very long title handling

### 3.3 Test TodoItem Edge Cases
- [ ] Test dengan input yang invalid:
  - Empty strings
  - Null/None values
  - Very long strings
  - Special characters
- [ ] Test serialization/deserialization (jika implementasi)
- [ ] Test Clone dan Debug traits
- [ ] Test equality dan comparison

## Task 4: TDD - Todo List Manager

### 4.1 Test Todo List Creation
- [ ] Buat file `src/todo_list.rs`
- [ ] Test `TodoList::new()` - empty list
- [ ] Test `TodoList::with_capacity()` - pre-allocated capacity
- [ ] Test initial state properties
- [ ] Implement minimal code to pass tests

### 4.2 Test Add Functionality
- [ ] Test `add_todo()` method:
  - Test adding single item
  - Test return value (success/failure)
  - Test list size increment
  - Test item is properly stored
- [ ] Test adding multiple items
- [ ] Test adding duplicate items (jika ada constraint)
- [ ] Test adding to full list (jika ada limit)

### 4.3 Test Remove Functionality
- [ ] Test `remove_todo()` by ID:
  - Test successful removal
  - Test removing non-existent ID
  - Test return value (Option/Result)
  - Test list size decrement
- [ ] Test `remove_todo_by_index()`:
  - Test valid index
  - Test invalid index (out of bounds)
  - Test boundary conditions (0, last index)

### 4.4 Test Mark Complete/Incomplete
- [ ] Test `mark_completed()` by ID:
  - Test successful marking
  - Test non-existent ID
  - Test already completed item
- [ ] Test `mark_all_completed()`
- [ ] Test `mark_all_incomplete()`
- [ ] Test batch operations

## Task 5: Advanced Testing - Query Methods

### 5.1 Test Filter Methods
- [ ] Test `get_completed_todos()`:
  - Test empty list
  - Test list with only incomplete items
  - Test list with only completed items
  - Test mixed list
- [ ] Test `get_incomplete_todos()`
- [ ] Test `count_completed()` dan `count_incomplete()`

### 5.2 Test Search Functionality
- [ ] Test `find_by_title()`:
  - Test exact match
  - Test partial match
  - Test case sensitivity
  - Test no matches
- [ ] Test `find_by_id()`:
  - Test existing ID
  - Test non-existent ID
  - Test edge case IDs (0, negative, very large)

### 5.3 Test Sorting dan Ordering
- [ ] Test `sort_by_title()`:
  - Test alphabetical sorting
  - Test empty list
  - Test single item
- [ ] Test `sort_by_completion_status()`
- [ ] Test `sort_by_creation_date()` (jika implementasi)

## Task 6: Error Handling Testing

### 6.1 Custom Error Types
- [ ] Buat file `src/error.rs`
- [ ] Definisi custom error enum `TodoError`
- [ ] Test error creation dan properties
- [ ] Test error message formatting
- [ ] Test error conversion (`From` implementations)

### 6.2 Test Error Scenarios
- [ ] Test operations pada empty list
- [ ] Test invalid IDs
- [ ] Test boundary conditions
- [ ] Test resource exhaustion (jika applicable)
- [ ] Test concurrent access issues (jika applicable)

### 6.3 Result<T, E> Testing
- [ ] Refactor methods untuk return `Result<T, TodoError>`
- [ ] Test success cases dengan `assert!(result.is_ok())`
- [ ] Test error cases dengan `assert!(result.is_err())`
- [ ] Test specific error types dengan `assert_matches!`
- [ ] Test error propagation dalam chain operations

## Task 7: Property-Based Testing

### 7.1 Setup Property Testing
- [ ] Tambahkan dependency `proptest = "1.0"` ke Cargo.toml
- [ ] Understand property-based testing concept
- [ ] Buat first property test untuk TodoItem

### 7.2 Property Tests untuk TodoItem
- [ ] Property: "Marking item as complete then incomplete returns to original state"
- [ ] Property: "Title update always changes the title field"
- [ ] Property: "New items are always incomplete"
- [ ] Property: "ID is always preserved through operations"

### 7.3 Property Tests untuk TodoList
- [ ] Property: "Adding then removing item returns list to original size"
- [ ] Property: "Count of completed + incomplete equals total count"
- [ ] Property: "Sorting doesn't change the number of items"
- [ ] Property: "Filter operations return subset of original list"

## Task 8: Performance Testing

### 8.1 Benchmark Setup
- [ ] Tambahkan dependency `criterion = "0.5"` ke Cargo.toml
- [ ] Setup benchmark directory `benches/`
- [ ] Buat first benchmark untuk basic operations

### 8.2 Benchmark Core Operations
- [ ] Benchmark `add_todo()` performance
- [ ] Benchmark `remove_todo()` performance
- [ ] Benchmark search operations
- [ ] Benchmark sorting operations
- [ ] Compare performance dengan different data sizes

### 8.3 Memory Usage Testing
- [ ] Test memory usage dengan large datasets
- [ ] Test for memory leaks
- [ ] Profile memory allocation patterns
- [ ] Test performance degradation over time

## Task 9: Mock Testing dan Dependency Injection

### 9.1 Setup Persistence Layer
- [ ] Buat trait `TodoStorage` untuk abstraction
- [ ] Implementasi `FileStorage` dan `MemoryStorage`
- [ ] Test both implementations
- [ ] Setup dependency injection pattern

### 9.2 Mock Testing dengan Mockall
- [ ] Create mock implementation of `TodoStorage`
- [ ] Test TodoList dengan mock storage
- [ ] Test error scenarios dengan mock
- [ ] Test call counts dan argument verification

### 9.3 Integration Testing
- [ ] Create integration tests di `tests/` directory
- [ ] Test complete workflows end-to-end
- [ ] Test persistence across application restarts
- [ ] Test data integrity

## Task 10: CLI Testing

### 10.1 CLI Structure
- [ ] Tambahkan dependency `clap = "4.0"`
- [ ] Setup basic CLI commands
- [ ] Separate CLI logic dari business logic
- [ ] Make CLI testable

### 10.2 CLI Unit Tests
- [ ] Test argument parsing
- [ ] Test command validation
- [ ] Test help text generation
- [ ] Test error handling dalam CLI

### 10.3 CLI Integration Tests
- [ ] Test complete CLI workflows
- [ ] Test output formatting
- [ ] Test file I/O operations
- [ ] Test error messages dan exit codes

## Task 11: Advanced Testing Techniques

### 11.1 Concurrent Testing
- [ ] Test thread safety (jika applicable)
- [ ] Test race conditions
- [ ] Test deadlock scenarios
- [ ] Use `std::sync::Arc` dan `Mutex` untuk testing

### 11.2 Fuzzing
- [ ] Setup fuzzing dengan `cargo-fuzz`
- [ ] Create fuzz targets untuk core functions
- [ ] Run fuzzing campaigns
- [ ] Fix issues found by fuzzing

### 11.3 Test Documentation
- [ ] Write doctests untuk all public APIs
- [ ] Test examples dalam documentation
- [ ] Ensure examples are up-to-date
- [ ] Test README examples

## Task 12: Test Organization dan Best Practices

### 12.1 Test Structure Best Practices
- [ ] Organize tests by functionality
- [ ] Use descriptive test names
- [ ] Follow Given-When-Then pattern
- [ ] Keep tests independent dan isolated

### 12.2 Test Data Management
- [ ] Create test fixtures dan helpers
- [ ] Setup dan teardown procedures
- [ ] Test data builders/factories
- [ ] Avoid test data pollution

### 12.3 Continuous Testing
- [ ] Setup pre-commit hooks
- [ ] Configure GitHub Actions/CI pipeline
- [ ] Setup test coverage reporting
- [ ] Monitor test performance over time

## Learning Goals - Testing Concepts

Pastikan Anda memahami konsep-konsep testing berikut:

### Basic Testing
- [ ] Unit vs Integration vs End-to-End tests
- [ ] Test structure (Arrange, Act, Assert)
- [ ] Test naming conventions
- [ ] When to test dan what not to test

### Rust-Specific Testing
- [ ] `#[test]` attribute dan variations
- [ ] `assert!` family macros
- [ ] `#[cfg(test)]` conditional compilation
- [ ] `cargo test` command dan options

### Advanced Testing
- [ ] Test doubles (mocks, stubs, fakes)
- [ ] Property-based testing
- [ ] Benchmark testing
- [ ] Fuzzing

### Test-Driven Development
- [ ] Red-Green-Refactor cycle
- [ ] Writing tests before implementation
- [ ] Refactoring dengan confidence
- [ ] Test as documentation

## Code Coverage dan Quality

### Coverage Setup
- [ ] Install `cargo-tarpaulin`: `cargo install cargo-tarpaulin`
- [ ] Run coverage: `cargo tarpaulin --out html`
- [ ] Aim for >90% test coverage
- [ ] Identify untested code paths

### Quality Metrics
- [ ] Setup `cargo clippy` untuk linting
- [ ] Use `cargo fmt` untuk formatting
- [ ] Run `cargo audit` untuk security
- [ ] Monitor test execution time

## Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo_item_has_correct_defaults() {
        // Given
        let title = "Test Todo";
        let description = Some("Test Description".to_string());
        
        // When
        let todo = TodoItem::new(title, description);
        
        // Then
        assert_eq!(todo.title(), title);
        assert_eq!(todo.description(), &description);
        assert!(!todo.is_completed());
        assert!(todo.id() > 0);
    }
    
    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn test_new_todo_item_panics_with_empty_title() {
        TodoItem::new("", None);
    }
}
```

---

## Getting Started

1. **Mulai dengan Task 1-2** untuk memahami testing basics
2. **Praktik TDD** mulai dari Task 3 - tulis test dulu, baru implementasi
3. **Jangan skip property testing** - ini sangat powerful untuk finding edge cases
4. **Focus pada test readability** - test adalah dokumentasi hidup
5. **Measure coverage** - tapi jangan obsess dengan 100%

Selamat belajar Rust Unit Testing! 🦀🧪

**Pro Tips:**
- Jalankan `cargo test` frequently
- Use `cargo test -- --nocapture` untuk debugging
- Write descriptive test names
- Test both happy path dan error cases
- Keep tests simple dan focused