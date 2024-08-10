use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ChatHistory {
    history_files: Vec<String>,
    directory: String,
    current_file: Option<String>,
}

impl ChatHistory {
    pub fn new(directory: &str) -> Self {
        let mut chat_history = Self {
            history_files: Vec::new(),
            directory: directory.to_string(),
            current_file: None,
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
        self.history_files.sort_by(|a, b| b.cmp(a));
    }

    pub fn get_history_files(&self) -> &Vec<String> {
        &self.history_files
    }

    pub fn create_new_chat(&mut self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.directory)?;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let file_name = format!("{}.txt", timestamp);
        let file_path = Path::new(&self.directory).join(&file_name);
        
        File::create(&file_path)?;
        self.current_file = Some(file_name.clone());
        self.load_history();
        
        Ok(())
    }

    pub fn append_message(&mut self, content: &str, is_user: bool) -> Result<(), std::io::Error> {
        if let Some(current_file) = &self.current_file {
            let file_path = Path::new(&self.directory).join(current_file);
            let mut file = fs::OpenOptions::new().append(true).open(file_path)?;
            let prefix = if is_user { "User: " } else { "Bot: " };
            writeln!(file, "{}{}", prefix, content)?;
        }
        Ok(())
    }

    pub fn load_chat(&mut self, file_name: &str) -> Result<String, std::io::Error> {
        let file_path = Path::new(&self.directory).join(file_name);
        let mut content = String::new();
        File::open(file_path)?.read_to_string(&mut content)?;
        self.current_file = Some(file_name.to_string());
        Ok(content)
    }
    pub fn delete_chat(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        let file_path = Path::new(&self.directory).join(file_name);
        fs::remove_file(&file_path)?;
        self.load_history();
        if self.current_file.as_ref().map_or(false, |f| f == file_name) {
            self.current_file = None;
        }
        Ok(())
    }
    pub fn get_current_file(&self) -> Option<&String> {
        self.current_file.as_ref()
    }
}