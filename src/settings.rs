use keyring::Entry;
use eframe::egui;
use std::time::Duration;

pub struct Settings {
    api_key: String,
    keyring: Entry,
    show_settings: bool,
    feedback: Option<(String, f32)>, // Changed to f32 for easier time handling
}

impl Settings {
    pub fn new() -> Self {
        let keyring = Entry::new("rusty_chatbot", "api_key").expect("Failed to create keyring entry");
        let api_key = keyring.get_password().unwrap_or_default();
        Self {
            api_key,
            keyring,
            show_settings: false,
            feedback: None,
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        if self.show_settings {
            egui::Window::new("Settings")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("API Key");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("✕").clicked() {
                                self.show_settings = false;
                            }
                        });
                    });
                    ui.text_edit_singleline(&mut self.api_key);
                    if ui.button("Save").clicked() {
                        match self.keyring.set_password(&self.api_key) {
                            Ok(_) => self.set_feedback("API key saved successfully.".to_string(), 3.0),
                            Err(_) => self.set_feedback("Failed to save API key.".to_string(), 3.0),
                        }
                    }
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

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    fn set_feedback(&mut self, message: String, duration: f32) {
        self.feedback = Some((message, duration));
    }
}