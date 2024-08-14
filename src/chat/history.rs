use crate::message::Message;
use std::fs::{self, File, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const MESSAGE_SEPARATOR: &str = "\n<<<MESSAGE_SEPARATOR>>>\n";

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

    fn load_history(&mut self) {
        self.history_files = fs::read_dir(&self.directory)
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
    }

    pub fn get_history_files(&self) -> Vec<String> {
        self.history_files.clone()
    }

    pub fn create_new_chat(&mut self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.directory)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .as_secs();
        let file_name = format!("{}.txt", timestamp);
        File::create(Path::new(&self.directory).join(&file_name))?;
        self.current_file = Some(file_name);
        self.load_history();
        Ok(())
    }

    pub fn append_message(&mut self, content: &str, is_user: bool) -> Result<(), std::io::Error> {
        if let Some(current_file) = &self.current_file {
            let mut file = OpenOptions::new().append(true).open(Path::new(&self.directory).join(current_file))?;
            writeln!(file, "{}{}{}", if is_user { "User: " } else { "Bot: " }, content, MESSAGE_SEPARATOR)?;
        }
        Ok(())
    }

    pub fn load_chat(&mut self, file_name: &str, messages: &mut Vec<Message>) -> Result<(), std::io::Error> {
        let mut content = String::new();
        File::open(Path::new(&self.directory).join(file_name))?.read_to_string(&mut content)?;
        self.current_file = Some(file_name.to_string());
        messages.clear();
        for message in content.split(MESSAGE_SEPARATOR) {
            let trimmed = message.trim();
            if !trimmed.is_empty() {
                if let Some(content) = trimmed.strip_prefix("User: ") {
                    messages.push(Message::new(content.to_string(), true));
                } else if let Some(content) = trimmed.strip_prefix("Bot: ") {
                    messages.push(Message::new(content.to_string(), false));
                }
            }
        }
        Ok(())
    }

    pub fn delete_chat(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        fs::remove_file(Path::new(&self.directory).join(file_name))?;
        self.load_history();
        if self.current_file.as_ref().map_or(false, |f| f == file_name) {
            self.current_file = None;
        }
        Ok(())
    }

    pub fn get_current_file(&self) -> Option<String> {
        self.current_file.clone()
    }

    pub fn export_chat(&self, path: &Path, messages: &[Message]) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        for message in messages {
            writeln!(file, "{}{}{}", if message.is_user() { "User: " } else { "Bot: " }, message.content(), MESSAGE_SEPARATOR)?;
        }
        Ok(())
    }
}