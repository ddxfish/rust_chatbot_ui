use keyring::Entry;
use super::Settings;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const SETTINGS_FILE: &str = "settings.ini";

pub fn load_api_keys(settings: &mut Settings) {
    let mut keys = settings.api_keys.lock().unwrap();
    if let Ok(entry) = Entry::new("rust_chatbot", "fireworks_api_key") {
        keys.fireworks = entry.get_password().unwrap_or_default();
    }
    if let Ok(entry) = Entry::new("rust_chatbot", "claude_api_key") {
        keys.claude = entry.get_password().unwrap_or_default();
    }
    if let Ok(entry) = Entry::new("rust_chatbot", "gpt_api_key") {
        keys.gpt = entry.get_password().unwrap_or_default();
    }
}

pub fn save_api_keys(settings: &mut Settings) {
    let keys = settings.api_keys.lock().unwrap();
    if let Ok(entry) = Entry::new("rust_chatbot", "fireworks_api_key") {
        let _ = entry.set_password(&keys.fireworks);
    }
    if let Ok(entry) = Entry::new("rust_chatbot", "claude_api_key") {
        let _ = entry.set_password(&keys.claude);
    }
    if let Ok(entry) = Entry::new("rust_chatbot", "gpt_api_key") {
        let _ = entry.set_password(&keys.gpt);
    }
    settings.api_keys_updated = true;
}

pub fn save_theme(settings: &Settings) {
    let content = format!("theme={}", settings.current_theme_index);
    if let Err(e) = File::create(SETTINGS_FILE).and_then(|mut file| file.write_all(content.as_bytes())) {
        eprintln!("Failed to save theme: {}", e);
    }
}

pub fn load_theme(settings: &mut Settings) {
    if let Ok(mut file) = File::open(SETTINGS_FILE) {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            if let Some(theme_index) = content.strip_prefix("theme=") {
                if let Ok(index) = theme_index.parse::<usize>() {
                    settings.current_theme_index = index.min(settings.themes.len().saturating_sub(1));
                }
            }
        }
    }
}
