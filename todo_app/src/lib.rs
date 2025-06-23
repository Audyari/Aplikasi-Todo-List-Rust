// src/lib.rs
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(id: u32, title: &str) -> Self {
        TodoItem {
            id,
            title: title.to_string(),
            completed: false,
        }
    }
}

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            items: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_item(&mut self, title: &str) {
        if title.trim().is_empty() {
            panic!("Title tidak boleh kosong");
        }
        
        let item = TodoItem::new(self.next_id, title);
        self.items.push(item);
        self.next_id += 1;
    }

    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }

    pub fn mark_as_completed(&mut self, index: usize) -> bool {
        if let Some(item) = self.items.get_mut(index) {
            item.completed = true;
            true
        } else {
            false
        }
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        // Tulis header
        writeln!(&mut file, "id,completed,title")?;
        
        // Tulis setiap item
        for item in &self.items {
            writeln!(
                &mut file,
                "{},{},{}",
                item.id,
                if item.completed { "true" } else { "false" },
                item.title
            )?;
        }
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut list = TodoList::new();
        let mut lines = reader.lines();

        // Lewati header
        if let Some(Ok(header)) = lines.next() {
            if header != "id,completed,title" {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Format file tidak valid: header tidak sesuai",
                ));
            }
        }

        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            
            if parts.len() != 3 {
                continue; // Lewati baris yang tidak valid
            }

            let id = parts[0].parse::<u32>().unwrap_or(0);
            let completed = parts[1] == "true";
            let title = parts[2].to_string();

            let item = TodoItem {
                id,
                title,
                completed,
            };

            list.items.push(item);
            list.next_id = list.next_id.max(id + 1);
        }

        Ok(list)
    }
}