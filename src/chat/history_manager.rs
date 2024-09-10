use crate::message::Message;
use std::path::Path;
use super::file_operations::{load_messages_from_file, append_message_to_file, create_new_chat_file, delete_chat_file, rename_chat_file};

pub struct ChatHistory {
    history_files: Vec<String>,
    directory: String,
    current_file: Option<String>,
}

impl ChatHistory {
    pub fn new(directory: &str) -> Self {
        println!("Debug: Creating new ChatHistory with directory: {}", directory);
        let mut chat_history = Self {
            history_files: Vec::new(),
            directory: directory.to_string(),
            current_file: None,
        };
        chat_history.load_history();
        chat_history
    }

    pub fn load_history(&mut self) {
        println!("Debug: Loading chat history");
        self.history_files = std::fs::read_dir(&self.directory)
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let file_name = e.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(".txt") {
                        Some(file_name)
                    } else {
                        None
                    }
                })
            })
            .collect();
        self.history_files.sort_by(|a, b| b.cmp(a));
        println!("Debug: Loaded {} chat files", self.history_files.len());
    }

    pub fn get_history_files(&self) -> Vec<String> {
        self.history_files.clone()
    }

    pub fn create_new_chat(&mut self) -> Result<String, std::io::Error> {
        println!("Debug: Creating new chat");
        let file_name = create_new_chat_file(&self.directory)?;
        self.current_file = Some(file_name.clone());
        self.load_history();
        println!("Debug: Created new chat file: {}", file_name);
        Ok(file_name)
    }

    pub fn append_message(&mut self, content: &str, is_user: bool, model: Option<&str>) -> Result<(), std::io::Error> {
        if let Some(current_file) = &self.current_file {
            println!("Debug: Appending message to file: {}", current_file);
            append_message_to_file(Path::new(&self.directory).join(current_file), content, is_user, model)?;
        } else {
            println!("Debug: No current file to append message");
        }
        Ok(())
    }

    pub fn load_chat(&mut self, file_name: &str, messages: &mut Vec<Message>) -> Result<(), std::io::Error> {
        println!("Debug: Loading chat from file: {}", file_name);
        load_messages_from_file(Path::new(&self.directory).join(file_name), messages)?;
        self.current_file = Some(file_name.to_string());
        println!("Debug: Loaded {} messages from file", messages.len());
        Ok(())
    }

    pub fn delete_chat(&mut self, file_name: &str) -> Result<Option<String>, std::io::Error> {
        println!("Debug: Deleting chat file: {}", file_name);
        delete_chat_file(Path::new(&self.directory).join(file_name))?;
        self.load_history();
        if self.current_file.as_ref().map_or(false, |f| f == file_name) {
            self.current_file = None;
            println!("Debug: Cleared current file as it was deleted");
        }
        if self.history_files.is_empty() {
            let new_file = self.create_new_chat()?;
            Ok(Some(new_file))
        } else {
            Ok(None)
        }
    }

    pub fn get_current_file(&self) -> Option<String> {
        self.current_file.clone()
    }

    pub fn rename_current_chat(&mut self, new_name: &str) -> Result<(), std::io::Error> {
        println!("Debug: Attempting to rename current chat to: {}", new_name);
        if let Some(current_file) = &self.current_file {
            let old_path = Path::new(&self.directory).join(current_file);
            let new_file_name = rename_chat_file(&old_path, new_name, &self.directory)?;
            self.current_file = Some(new_file_name);
            self.load_history();
            println!("Debug: Successfully renamed chat file");
        } else {
            println!("Debug: No current file to rename");
        }
        Ok(())
    }
}