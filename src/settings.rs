use eframe::egui::{self, Window, RichText};
use crate::app::Icons;
use crate::ui::theme::{Theme, get_themes};
use crate::providers::Provider;
use std::sync::{Arc, Mutex};
use keyring::Entry;

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
        settings.load_api_keys();
        settings
    }

    fn load_api_keys(&mut self) {
        let mut keys = self.api_keys.lock().unwrap();
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

    fn save_api_keys(&mut self) {
        let keys = self.api_keys.lock().unwrap();
        if let Ok(entry) = Entry::new("rust_chatbot", "fireworks_api_key") {
            let _ = entry.set_password(&keys.fireworks);
        }
        if let Ok(entry) = Entry::new("rust_chatbot", "claude_api_key") {
            let _ = entry.set_password(&keys.claude);
        }
        if let Ok(entry) = Entry::new("rust_chatbot", "gpt_api_key") {
            let _ = entry.set_password(&keys.gpt);
        }
        self.api_keys_updated = true;
    }

    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
    }

    pub fn render(&mut self, ctx: &egui::Context, icons: &Icons) {
        if self.show_settings {
            let mut save_clicked = false;
            Window::new("Settings")
                .collapsible(false)
                .resizable(false)
                //.title_bar_background_color(self.themes[self.current_theme_index].settings_window_title_bg_color)
                .show(ctx, |ui| {
                    let theme = &self.themes[self.current_theme_index];
                    let mut keys = self.api_keys.lock().unwrap();

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Fireworks API Key:").strong().color(theme.settings_text_color));
                        ui.text_edit_singleline(&mut keys.fireworks);
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Claude API Key:").strong().color(theme.settings_text_color));
                        ui.text_edit_singleline(&mut keys.claude);
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("GPT API Key:").strong().color(theme.settings_text_color));
                        ui.text_edit_singleline(&mut keys.gpt);
                    });

                    if ui.add(egui::Button::new(RichText::new("Save API Keys").color(theme.settings_button_text_color)).fill(theme.settings_button_bg_color)).clicked() {
                        save_clicked = true;
                    }

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Theme:").strong().color(theme.settings_text_color));
                        egui::ComboBox::from_label("")
                            .selected_text(RichText::new(&self.themes[self.current_theme_index].name).color(theme.dropdown_text_color))
                            .show_ui(ui, |ui| {
                                for (index, theme) in self.themes.iter().enumerate() {
                                    ui.selectable_value(&mut self.current_theme_index, index, RichText::new(&theme.name).color(theme.dropdown_text_color));
                                }
                            });
                    });

                    ui.add_space(10.0);

                    if ui.add(egui::Button::new(RichText::new("Close").color(theme.settings_button_text_color)).fill(theme.settings_button_bg_color)).clicked() {
                        self.show_settings = false;
                    }
                });

            if save_clicked {
                self.save_api_keys();
            }
        }
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