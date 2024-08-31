use eframe::egui::{self, Window, RichText};
use crate::app::Icons;
use crate::ui::theme::{Theme, get_themes};
use crate::providers::Provider;
use std::sync::Arc;
use keyring::Entry;

pub struct Settings {
    pub show_settings: bool,
    fireworks_api_key: String,
    claude_api_key: String,
    gpt_api_key: String,
    themes: Vec<Theme>,
    current_theme_index: usize,
    pub api_keys_updated: bool,
}

impl Settings {
    pub fn new() -> Self {
        let themes = get_themes();
        let mut settings = Self {
            show_settings: false,
            fireworks_api_key: String::new(),
            claude_api_key: String::new(),
            gpt_api_key: String::new(),
            themes,
            current_theme_index: 0,
            api_keys_updated: false,
        };
        settings.load_api_keys();
        settings
    }

    fn load_api_keys(&mut self) {
        if let Ok(entry) = Entry::new("rust_chatbot", "fireworks_api_key") {
            self.fireworks_api_key = entry.get_password().unwrap_or_default();
        }
        if let Ok(entry) = Entry::new("rust_chatbot", "claude_api_key") {
            self.claude_api_key = entry.get_password().unwrap_or_default();
        }
        if let Ok(entry) = Entry::new("rust_chatbot", "gpt_api_key") {
            self.gpt_api_key = entry.get_password().unwrap_or_default();
        }
    }

    fn save_api_keys(&mut self) {
        if let Ok(entry) = Entry::new("rust_chatbot", "fireworks_api_key") {
            let _ = entry.set_password(&self.fireworks_api_key);
        }
        if let Ok(entry) = Entry::new("rust_chatbot", "claude_api_key") {
            let _ = entry.set_password(&self.claude_api_key);
        }
        if let Ok(entry) = Entry::new("rust_chatbot", "gpt_api_key") {
            let _ = entry.set_password(&self.gpt_api_key);
        }
        self.api_keys_updated = true;
    }

    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
    }

    pub fn render(&mut self, ctx: &egui::Context, icons: &Icons) {
        if self.show_settings {
            Window::new("Settings")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Fireworks API Key:").strong());
                        ui.text_edit_singleline(&mut self.fireworks_api_key);
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Claude API Key:").strong());
                        ui.text_edit_singleline(&mut self.claude_api_key);
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("GPT API Key:").strong());
                        ui.text_edit_singleline(&mut self.gpt_api_key);
                    });

                    if ui.button("Save API Keys").clicked() {
                        self.save_api_keys();
                    }

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Theme:").strong());
                        egui::ComboBox::from_label("")
                            .selected_text(&self.themes[self.current_theme_index].name)
                            .show_ui(ui, |ui| {
                                for (index, theme) in self.themes.iter().enumerate() {
                                    ui.selectable_value(&mut self.current_theme_index, index, &theme.name);
                                }
                            });
                    });

                    ui.add_space(10.0);

                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
        }
    }

    pub fn get_api_keys(&self) -> String {
        format!("{},{},{}", self.fireworks_api_key, self.claude_api_key, self.gpt_api_key)
    }

    pub fn get_current_theme(&self) -> &Theme {
        &self.themes[self.current_theme_index]
    }

    pub fn get_first_provider_with_key(&self, providers: &[Arc<dyn Provider + Send + Sync>]) -> Arc<dyn Provider + Send + Sync> {
        if !self.fireworks_api_key.is_empty() {
            return Arc::clone(&providers[1]);
        } else if !self.claude_api_key.is_empty() {
            return Arc::clone(&providers[2]);
        } else if !self.gpt_api_key.is_empty() {
            return Arc::clone(&providers[3]);
        }
        Arc::clone(&providers[0])
    }
}