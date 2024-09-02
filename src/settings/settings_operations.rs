use keyring::Entry;
use super::Settings;

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