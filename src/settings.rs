use keyring::Entry;
use eframe::egui;
use egui::{Button, Image, Vec2};
use crate::app::Icons;

pub struct Settings {
    fireworks_api_key: String,
    keyring: Entry,
    show_settings: bool,
    feedback: Option<(String, f32)>,
}

impl Settings {
    pub fn new() -> Self {
        let keyring = Entry::new("rusty_chatbot", "fireworks_api_key").expect("Failed to create keyring entry");
        let fireworks_api_key = keyring.get_password().unwrap_or_default();
        Self {
            fireworks_api_key,
            keyring,
            show_settings: false,
            feedback: None,
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, icons: &Icons) {
        if self.show_settings {
            egui::Window::new("Settings")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Fireworks API Key");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(Button::image(Image::new(&icons.close).fit_to_exact_size(Vec2::new(20.0, 20.0)))).clicked() {
                                self.show_settings = false;
                            }
                        });
                    });
                    ui.text_edit_singleline(&mut self.fireworks_api_key);
                    if ui.button("Save").clicked() {
                        match self.keyring.set_password(&self.fireworks_api_key) {
                            Ok(_) => self.set_feedback("Fireworks API key saved successfully.".to_string(), 3.0),
                            Err(_) => self.set_feedback("Failed to save Fireworks API key.".to_string(), 3.0),
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

    pub fn get_fireworks_api_key(&self) -> &str {
        &self.fireworks_api_key
    }

    fn set_feedback(&mut self, message: String, duration: f32) {
        self.feedback = Some((message, duration));
    }
}