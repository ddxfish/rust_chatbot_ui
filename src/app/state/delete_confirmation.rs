use eframe::egui;
use crate::chat::Chat;

pub struct DeleteConfirmation {
    file_to_delete: Option<String>,
}

impl DeleteConfirmation {
    pub fn new() -> Self {
        Self {
            file_to_delete: None,
        }
    }

    pub fn set_file_to_delete(&mut self, file: String) {
        self.file_to_delete = Some(file);
    }

    pub fn handle_confirmation(&mut self, ctx: &egui::Context, chat: &mut Chat) {
        let mut action = None;

        if let Some(file) = &self.file_to_delete {
            egui::Window::new("Confirm Delete")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("Are you sure you want to delete '{}'?", file));
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            action = Some(true);
                        }
                        if ui.button("No").clicked() {
                            action = Some(false);
                        }
                    });
                });
        }

        if let Some(true) = action {
            if let Some(file) = self.file_to_delete.take() {
                if let Err(e) = chat.delete_chat(&file) {
                    eprintln!("Failed to delete chat: {}", e);
                }
            }
        } else if let Some(false) = action {
            self.file_to_delete = None;
        }
    }
}