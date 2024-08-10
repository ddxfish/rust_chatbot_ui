use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ChatHistory {
    history_files: Vec<String>,
    directory: String,
}

impl ChatHistory {
    pub fn new(directory: &str) -> Self {
        let mut chat_history = Self {
            history_files: Vec::new(),
            directory: directory.to_string(),
        };
        chat_history.load_history();
        chat_history
    }

    pub fn load_history(&mut self) {
        self.history_files.clear();
        if let Ok(entries) = fs::read_dir(&self.directory) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".txt") {
                        self.history_files.push(file_name.to_string());
                    }
                }
            }
        }
    }

    pub fn get_history_files(&mut self) -> &mut Vec<String> {
        &mut self.history_files
    }

    pub fn create_new_chat(&mut self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.directory)?;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let file_name = format!("{}.txt", timestamp);
        let file_path = Path::new(&self.directory).join(&file_name);
        
        fs::File::create(file_path)?;
        self.load_history();
        
        Ok(())
    }
}