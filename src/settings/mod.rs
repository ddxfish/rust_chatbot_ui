mod settings_ui;
mod settings_operations;

use crate::ui::themes::{Theme, get_themes};
use crate::providers::Provider;
use std::sync::{Arc, Mutex};

pub struct Settings {
    pub show_settings: bool,
    api_keys: Arc<Mutex<ApiKeys>>,
    themes: Vec<Theme>,
    current_theme_index: usize,
    pub api_keys_updated: bool,
}

struct ApiKeys {
    fireworks: String,
    claude: String,
    gpt: String,
}

impl Settings {
    pub fn new() -> Self {
        let themes = get_themes();
        let api_keys = Arc::new(Mutex::new(ApiKeys {
            fireworks: String::new(),
            claude: String::new(),
            gpt: String::new(),
        }));
        let mut settings = Self {
            show_settings: false,
            api_keys,
            themes,
            current_theme_index: 0,
            api_keys_updated: false,
        };
        settings_operations::load_api_keys(&mut settings);
        settings_operations::load_theme(&mut settings);
        settings
    }

    pub fn get_api_keys(&self) -> String {
        let keys = self.api_keys.lock().unwrap();
        format!("{},{},{}", keys.fireworks, keys.claude, keys.gpt)
    }

    pub fn get_current_theme(&self) -> &Theme {
        &self.themes[self.current_theme_index]
    }

    pub fn get_first_provider_with_key(&self, providers: &[Arc<dyn Provider + Send + Sync>]) -> Arc<dyn Provider + Send + Sync> {
        let keys = self.api_keys.lock().unwrap();
        if !keys.fireworks.is_empty() {
            return Arc::clone(&providers[1]);
        } else if !keys.claude.is_empty() {
            return Arc::clone(&providers[2]);
        } else if !keys.gpt.is_empty() {
            return Arc::clone(&providers[3]);
        }
        Arc::clone(&providers[0])
    }
}

pub use settings_ui::render;
pub use settings_operations::{load_api_keys, save_api_keys, save_theme, load_theme};