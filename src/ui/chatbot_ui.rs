use egui::{Ui, ScrollArea, Align, Layout};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::app::Icons;
use super::{message_view, input_area, bottom_panel};

pub struct ChatbotUi {
    input: String,
    selected_provider: String,
    selected_model: String,
    is_loading: bool,
}

impl ChatbotUi {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            selected_provider: String::new(),
            selected_model: String::new(),
            is_loading: false,
        }
    }

    pub fn render(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, icons: &Icons) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let available_height = ui.available_height();
            let bottom_row_height = 30.0;
            let input_height = 45.0;
            let bottom_padding = 0.0;
            let message_height = available_height - input_height - bottom_row_height - bottom_padding;
            
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(message_height)
                .show(ui, |ui| {
                    message_view::render_messages(ui, chat);
                });
            
            input_area::render_input(ui, chat, icons, &mut self.input, &mut self.is_loading);
            
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                if bottom_padding > 0.0 {
                    ui.allocate_space(egui::vec2(ui.available_width(), bottom_padding));
                }
                
                bottom_panel::render(ui, chat, settings, &mut self.selected_provider, &mut self.selected_model);
            });
        });
    
        settings.render(ui.ctx(), icons);

        if chat.is_processing() {
            ui.ctx().request_repaint();
        } else if chat.check_response().is_some() {
            ui.ctx().request_repaint();
        }
    }
}