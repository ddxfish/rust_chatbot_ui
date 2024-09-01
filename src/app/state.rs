use crate::chat::Chat;
use crate::chat::history_manager::ChatHistory;
use crate::app::Icons;
use crate::settings::Settings;
use crate::ui::ChatbotUi;
use crate::providers::Provider;
use crate::ui::bottom_panel;
use crate::ui::themes::Theme;
use eframe::egui::{self, Ui, ScrollArea};
use std::sync::Arc;
use crate::chat::ui_rendering;
pub struct ChatbotAppState {
    pub chat_history: ChatHistory,
    pub previous_model: Option<String>,
}

impl ChatbotAppState {
    pub fn new() -> Self {
        Self {
            chat_history: ChatHistory::new("chat_history"),
            previous_model: None,
        }
    }

    pub fn update(&mut self, chat: &mut Chat) {
        self.chat_history.load_history();
    }


    pub fn render_chat_history(&mut self, ui: &mut egui::Ui, chat: &mut Chat, icons: &Icons, theme: &Theme) {
        ScrollArea::vertical().show(ui, |ui| {
            ui_rendering::render_history(
                ui,
                chat,
                icons,
                theme,
                &self.chat_history.get_history_files(),
                chat.get_current_file().as_ref()
            );
        });
    }

    pub fn render_bottom_left_section(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, chatbot_ui: &mut ChatbotUi, providers: &[Arc<dyn Provider + Send + Sync>], theme: &Theme) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            bottom_panel::render(ui, chat, settings, chatbot_ui, providers, theme);
        });
    }
}