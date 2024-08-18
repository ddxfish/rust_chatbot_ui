use egui::{Ui, ScrollArea, Align, Layout};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::app::Icons;
use super::{message_view, input_area};
use crate::providers::Provider;
use std::sync::Arc;

pub struct ChatbotUi {
    pub input: String,
    pub selected_provider: String,
    pub selected_model: String,
    pub is_loading: bool,
    pub current_response: String,
}

impl ChatbotUi {
    pub fn new(initial_provider: String, initial_model: String) -> Self {
        Self {
            input: String::new(),
            selected_provider: initial_provider,
            selected_model: initial_model,
            is_loading: false,
            current_response: String::new(),
        }
    }


    pub fn render(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, icons: &Icons, providers: &[Arc<dyn Provider + Send + Sync>]) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let available_height = ui.available_height();
            let input_height = 45.0;
            let bottom_padding = 20.0;
            let message_height = available_height - input_height - bottom_padding;
            
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(message_height)
                .show(ui, |ui| {
                    message_view::render_messages(ui, chat, &self.current_response, self.is_loading);
                });
            
            input_area::render_input(ui, chat, icons, &mut self.input, &mut self.is_loading);
            
            if bottom_padding > 0.0 {
                ui.allocate_space(egui::vec2(ui.available_width(), bottom_padding));
            }
            
        });
        
        settings.render(ui.ctx(), icons);

        if chat.is_processing() {
            self.is_loading = true;
        }

        while let Some(chunk) = chat.check_ui_updates() {
            self.current_response.push_str(&chunk);
            ui.ctx().request_repaint();
        }

        if !chat.is_processing() && !self.current_response.is_empty() {
            chat.add_message(std::mem::take(&mut self.current_response), false);
            self.is_loading = false;
        }

        if let Some(new_name) = chat.check_name_updates() {
            if let Err(e) = chat.rename_current_chat(&new_name) {
                eprintln!("Error: Failed to rename chat: {}", e);
            }
        }

        ui.ctx().request_repaint();
    }
}