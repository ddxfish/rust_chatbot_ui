use keyring::Entry;
use super::Settings;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::collections::HashMap;

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
    set_ini_value("Settings", "theme", &settings.current_theme_index.to_string());
}

pub fn load_theme(settings: &mut Settings) {
    if let Some(theme_index) = get_ini_value("Settings", "theme") {
        if let Ok(index) = theme_index.parse::<usize>() {
            settings.current_theme_index = index.min(settings.themes.len().saturating_sub(1));
        }
    }
}

pub fn save_ui_scale(settings: &Settings) {
    set_ini_value("Settings", "ui_scale", &settings.ui_scale.to_string());
}

pub fn load_ui_scale(settings: &mut Settings) {
    if let Some(ui_scale) = get_ini_value("Settings", "ui_scale") {
        if let Ok(scale) = ui_scale.parse::<f32>() {
            settings.ui_scale = scale.clamp(0.5, 2.0);
        }
    }
}

fn get_ini_value(section: &str, key: &str) -> Option<String> {
    let content = std::fs::read_to_string(SETTINGS_FILE).ok()?;
    let mut current_section = String::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
        } else if current_section == section {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 && parts[0].trim() == key {
                return Some(parts[1].trim().to_string());
            }
        }
    }
    None
}

fn set_ini_value(section: &str, key: &str, value: &str) {
    let path = Path::new(SETTINGS_FILE);
    let mut content = if path.exists() {
        std::fs::read_to_string(path).unwrap_or_default()
    } else {
        String::new()
    };

    let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
        } else if !current_section.is_empty() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                sections.entry(current_section.clone())
                    .or_insert_with(HashMap::new)
                    .insert(parts[0].trim().to_string(), parts[1].trim().to_string());
            }
        }
    }

    sections.entry(section.to_string())
        .or_insert_with(HashMap::new)
        .insert(key.to_string(), value.to_string());

    let mut new_content = String::new();
    for (section, keys) in sections {
        new_content.push_str(&format!("[{}]\n", section));
        for (key, value) in keys {
            new_content.push_str(&format!("{}={}\n", key, value));
        }
        new_content.push('\n');
    }

    if let Err(e) = std::fs::write(path, new_content) {
        eprintln!("Failed to write settings: {}", e);
    }
}