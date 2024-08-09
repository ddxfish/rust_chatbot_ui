use keyring::Entry;
use eframe::egui;

pub struct Settings {
    api_key: String,
    keyring: Entry,
    show_settings: bool,
}

impl Settings {
    pub fn new() -> Self {
        let keyring = Entry::new("rusty_chatbot", "api_key").expect("Failed to create keyring entry");
        let api_key = keyring.get_password().unwrap_or_default();
        Self {
            api_key,
            keyring,
            show_settings: false,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        if ui.button("Settings").clicked() {
            self.show_settings = !self.show_settings;
        }

        if self.show_settings {
            egui::Window::new("Settings").show(ui.ctx(), |ui| {
                ui.heading("API Key");
                ui.text_edit_singleline(&mut self.api_key);
                if ui.button("Save").clicked() {
                    self.keyring.set_password(&self.api_key).expect("Failed to save API key");
                }
            });
        }
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }
}