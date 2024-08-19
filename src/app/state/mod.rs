mod chat_history;
mod delete_confirmation;

use crate::chat::Chat;
use crate::app::Icons;
use crate::settings::Settings;
use crate::ui::ChatbotUi;
use crate::providers::Provider;
use crate::ui::bottom_panel;
use crate::ui::theme::DarkTheme;
use eframe::egui::{self, Ui, ScrollArea};
use std::sync::Arc;

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

    pub fn render_chat_history(&mut self, ui: &mut egui::Ui, chat: &mut Chat, icons: &Icons, theme: &DarkTheme) {
        ScrollArea::vertical().show(ui, |ui| {
            if let Some(file_to_delete) = self.chat_history.render(ui, chat, icons) {
                self.delete_confirmation.set_file_to_delete(file_to_delete);
            }
        });
    }

    pub fn render_bottom_left_section(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, chatbot_ui: &mut ChatbotUi, providers: &[Arc<dyn Provider + Send + Sync>], theme: &DarkTheme) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            bottom_panel::render(ui, chat, settings, chatbot_ui, providers, theme);
        });
    }

    pub fn handle_delete_confirmation(&mut self, ctx: &egui::Context, chat: &mut Chat) {
        self.delete_confirmation.handle_confirmation(ctx, chat);
    }
}