mod chat_history;

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
    pub chat_history: chat_history::ChatHistory,
    pub previous_model: Option<String>,
}

impl ChatbotAppState {
    pub fn new() -> Self {
        Self {
            chat_history: chat_history::ChatHistory::new(),
            previous_model: None,
        }
    }

    pub fn update(&mut self, chat: &mut Chat) {
        self.chat_history.update(chat);
    }

    pub fn render_chat_history(&mut self, ui: &mut egui::Ui, chat: &mut Chat, icons: &Icons, theme: &DarkTheme) {
        ScrollArea::vertical().show(ui, |ui| {
            self.chat_history.render(ui, chat, icons);
        });
    }

    pub fn render_bottom_left_section(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, chatbot_ui: &mut ChatbotUi, providers: &[Arc<dyn Provider + Send + Sync>], theme: &DarkTheme) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            bottom_panel::render(ui, chat, settings, chatbot_ui, providers, theme);
        });
    }
}