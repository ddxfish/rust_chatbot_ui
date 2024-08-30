use keyring::Entry;
use eframe::egui;
use egui::{Button, Image, Vec2, ComboBox};
use crate::app::Icons;
use crate::ui::theme::{Theme, get_themes};

pub struct Settings {
    fireworks_api_key: String,
    claude_api_key: String,
    keyring: Entry,
    pub show_settings: bool,
    feedback: Option<(String, f32)>,
    pub api_keys_updated: bool,
    pub themes: Vec<Theme>,
    pub current_theme_index: usize,
}

impl Settings {
    pub fn new() -> Self {
        let keyring = Entry::new("rusty_chatbot", "api_keys").expect("Failed to create keyring entry");
        let api_keys = keyring.get_password().unwrap_or_default();
        let keys: Vec<&str> = api_keys.split(',').collect();
        let fireworks_api_key = keys.get(0).unwrap_or(&"").to_string();
        let claude_api_key = keys.get(1).unwrap_or(&"").to_string();
        let themes = get_themes();
        Self {
            fireworks_api_key,
            claude_api_key,
            keyring,
            show_settings: false,
            feedback: None,
            api_keys_updated: false,
            themes,
            current_theme_index: 0,
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, icons: &Icons) {
        if self.show_settings {
            egui::Window::new("Settings")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Settings");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(Button::image(Image::new(&icons.close).fit_to_exact_size(Vec2::new(20.0, 20.0)))).clicked() {
                                self.show_settings = false;
                            }
                        });
                    });

                    ui.heading("Fireworks API Key");
                    ui.text_edit_singleline(&mut self.fireworks_api_key);

                    ui.heading("Claude API Key");
                    ui.text_edit_singleline(&mut self.claude_api_key);

                    if ui.button("Save API Keys").clicked() {
                        let api_keys = format!("{},{}", self.fireworks_api_key, self.claude_api_key);
                        match self.keyring.set_password(&api_keys) {
                            Ok(_) => {
                                self.set_feedback("API keys saved successfully.".to_string(), 3.0);
                                self.api_keys_updated = true;
                            },
                            Err(_) => self.set_feedback("Failed to save API keys.".to_string(), 3.0),
                        }
                    }

                    ui.add_space(10.0);
                    ui.heading("Theme");
                    ComboBox::from_label("Select Theme")
                        .selected_text(&self.themes[self.current_theme_index].name)
                        .show_ui(ui, |ui| {
                            for (index, theme) in self.themes.iter().enumerate() {
                                ui.selectable_value(&mut self.current_theme_index, index, &theme.name);
                            }
                        });

                    if let Some((message, _)) = &self.feedback {
                        ui.label(message);
                    }
                });
        }

        if let Some((_, remaining_time)) = &mut self.feedback {
            *remaining_time -= ctx.input(|i| i.unstable_dt);
            if *remaining_time <= 0.0 {
                self.feedback = None;
            }
        }
    }

    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
    }

    fn set_feedback(&mut self, message: String, duration: f32) {
        self.feedback = Some((message, duration));
    }

    pub fn get_api_keys(&self) -> String {
        format!("{},{}", self.fireworks_api_key, self.claude_api_key)
    }

    pub fn get_current_theme(&self) -> Theme {
        self.themes[self.current_theme_index].clone()
    }
}