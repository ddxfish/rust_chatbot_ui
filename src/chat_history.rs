use std::fs;
use std::path::Path;

pub struct ChatHistory {
    history_files: Vec<String>,
}

impl ChatHistory {
    pub fn new() -> Self {
        Self {
            history_files: Vec::new(),
        }
    }

    pub fn load_history(&mut self, directory: &str) {
        self.history_files.clear();
        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".txt") {
                        self.history_files.push(file_name.to_string());
                    }
                }
            }
        }
    }

    pub fn get_history_files(&self) -> &Vec<String> {
        &self.history_files
    }
}