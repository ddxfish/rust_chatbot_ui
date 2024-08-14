mod chat_history;
mod delete_confirmation;

use crate::chat::Chat;
use crate::app::Icons;
use eframe::egui;

pub struct ChatbotAppState {
    chat_history: chat_history::ChatHistory,
    delete_confirmation: delete_confirmation::DeleteConfirmation,
}

impl ChatbotAppState {
    pub fn new() -> Self {
        Self {
            chat_history: chat_history::ChatHistory::new(),
            delete_confirmation: delete_confirmation::DeleteConfirmation::new(),
        }
    }

    pub fn update(&mut self, chat: &mut Chat) {
        self.chat_history.update(chat);
    }

    pub fn render_chat_history(&mut self, ui: &mut egui::Ui, chat: &mut Chat, icons: &Icons) {
        if let Some(file_to_delete) = self.chat_history.render(ui, chat, icons) {
            self.delete_confirmation.set_file_to_delete(file_to_delete);
        }
    }

    pub fn handle_delete_confirmation(&mut self, ctx: &egui::Context, chat: &mut Chat) {
        self.delete_confirmation.handle_confirmation(ctx, chat);
    }
}