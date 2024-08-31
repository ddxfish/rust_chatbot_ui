use crate::message::Message;
use std::fs::{self, File, OpenOptions};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MESSAGE_SEPARATOR: &str = "\n<<<MESSAGE_SEPARATOR>>>\n";

pub fn create_new_chat_file(directory: &str) -> Result<String, std::io::Error> {
    fs::create_dir_all(directory)?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        .as_secs();
    let file_name = format!("{}.txt", timestamp);
    File::create(Path::new(directory).join(&file_name))?;
    Ok(file_name)
}

pub fn append_message_to_file(file_path: PathBuf, content: &str, is_user: bool, model: Option<&str>) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().append(true).open(file_path)?;
    let prefix = if is_user { 
        "User: ".to_string() 
    } else { 
        let model_name = match model {
            Some(m) if m.starts_with("accounts/fireworks/models/") => {
                m.split('/').last().unwrap_or("Custom Model")
            },
            Some(m) => m,
            None => "Bot"
        };
        format!("{}: ", model_name)
    };
    writeln!(file, "{}{}{}", prefix, content, MESSAGE_SEPARATOR)?;
    Ok(())
}

pub fn load_messages_from_file(file_path: PathBuf, messages: &mut Vec<Message>) -> Result<(), std::io::Error> {
    let mut content = String::new();
    File::open(file_path)?.read_to_string(&mut content)?;
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
    Ok(())
}

pub fn delete_chat_file(file_path: PathBuf) -> Result<(), std::io::Error> {
    fs::remove_file(file_path)
}

pub fn rename_chat_file(old_path: &Path, new_name: &str, directory: &str) -> Result<String, std::io::Error> {
    let mut new_name = new_name.to_string();
    new_name.retain(|c| c.is_alphanumeric() || c.is_whitespace());
    new_name = new_name.replace(" ", "_");

    let mut new_path = PathBuf::from(directory);
    new_path.push(format!("{}.txt", new_name));

    let mut counter = 1;
    while new_path.exists() {
        new_path.set_file_name(format!("{}_{}.txt", new_name, counter));
        counter += 1;
    }

    println!("Debug: Renaming from {:?} to {:?}", old_path, new_path);
    fs::rename(old_path, &new_path)?;
    Ok(new_path.file_name().unwrap().to_string_lossy().to_string())
}

pub fn export_chat(path: &Path, messages: &[Message]) -> Result<(), std::io::Error> {
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