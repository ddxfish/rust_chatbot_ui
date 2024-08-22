use crate::message::Message;
use std::fs::{self, File, OpenOptions};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MESSAGE_SEPARATOR: &str = "\n<<<MESSAGE_SEPARATOR>>>\n";

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

    fn load_history(&mut self) {
        println!("Debug: Loading chat history");
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
        println!("Debug: Loaded {} chat files", self.history_files.len());
    }

    pub fn get_history_files(&self) -> Vec<String> {
        self.history_files.clone()
    }

    pub fn create_new_chat(&mut self) -> Result<(), std::io::Error> {
        println!("Debug: Creating new chat");
        fs::create_dir_all(&self.directory)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .as_secs();
        let file_name = format!("{}.txt", timestamp);
        File::create(Path::new(&self.directory).join(&file_name))?;
        self.current_file = Some(file_name.clone());
        self.load_history();
        println!("Debug: Created new chat file: {}", file_name);
        Ok(())
    }

    pub fn append_message(&mut self, content: &str, is_user: bool, model: Option<&str>) -> Result<(), std::io::Error> {
        if let Some(current_file) = &self.current_file {
            println!("Debug: Appending message to file: {}", current_file);
            let mut file = OpenOptions::new().append(true).open(Path::new(&self.directory).join(current_file))?;
            let prefix = if is_user { 
                "User: ".to_string() 
            } else { 
                format!("{}: ", model.unwrap_or("Bot"))
            };
            writeln!(file, "{}{}{}", prefix, content, MESSAGE_SEPARATOR)?;
        } else {
            println!("Debug: No current file to append message");
        }
        Ok(())
    }

    pub fn load_chat(&mut self, file_name: &str, messages: &mut Vec<Message>) -> Result<(), std::io::Error> {
        println!("Debug: Loading chat from file: {}", file_name);
        let mut content = String::new();
        File::open(Path::new(&self.directory).join(file_name))?.read_to_string(&mut content)?;
        self.current_file = Some(file_name.to_string());
        messages.clear();
        for message in content.split(MESSAGE_SEPARATOR) {
            let trimmed = message.trim();
            if !trimmed.is_empty() {
                if let Some(content) = trimmed.strip_prefix("User: ") {
                    messages.push(Message::new(content.to_string(), true, None));
                } else if let Some((model, content)) = trimmed.split_once(": ") {
                    messages.push(Message::new(content.to_string(), false, Some(model.to_string())));
                }
            }
        }
        println!("Debug: Loaded {} messages from file", messages.len());
        Ok(())
    }

    pub fn delete_chat(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        println!("Debug: Deleting chat file: {}", file_name);
        fs::remove_file(Path::new(&self.directory).join(file_name))?;
        self.load_history();
        if self.current_file.as_ref().map_or(false, |f| f == file_name) {
            self.current_file = None;
            println!("Debug: Cleared current file as it was deleted");
        }
        Ok(())
    }

    pub fn get_current_file(&self) -> Option<String> {
        self.current_file.clone()
    }

    pub fn export_chat(&self, path: &Path, messages: &[Message]) -> Result<(), std::io::Error> {
        println!("Debug: Exporting chat to: {:?}", path);
        let mut file = File::create(path)?;
        for message in messages {
            let prefix = if message.is_user() {
                "User: ".to_string()
            } else {
                format!("{}: ", message.model().unwrap_or("Bot"))
            };
            writeln!(file, "{}{}{}", prefix, message.content(), MESSAGE_SEPARATOR)?;
        }
        println!("Debug: Exported {} messages", messages.len());
        Ok(())
    }

    pub fn rename_current_chat(&mut self, new_name: &str) -> Result<(), std::io::Error> {
        println!("Debug: Attempting to rename current chat to: {}", new_name);
        if let Some(current_file) = &self.current_file {
            let old_path = Path::new(&self.directory).join(current_file);
            let mut new_name = new_name.to_string();
            new_name.retain(|c| c.is_alphanumeric() || c.is_whitespace());
            new_name = new_name.replace(" ", "_");

            let mut new_path = PathBuf::from(&self.directory);
            new_path.push(format!("{}.txt", new_name));

            let mut counter = 1;
            while new_path.exists() {
                new_path.set_file_name(format!("{}_{}.txt", new_name, counter));
                counter += 1;
            }

            println!("Debug: Renaming from {:?} to {:?}", old_path, new_path);
            fs::rename(&old_path, &new_path)?;
            self.current_file = Some(new_path.file_name().unwrap().to_string_lossy().to_string());
            self.load_history();
            println!("Debug: Successfully renamed chat file");
        } else {
            println!("Debug: No current file to rename");
        }
        Ok(())
    }
}