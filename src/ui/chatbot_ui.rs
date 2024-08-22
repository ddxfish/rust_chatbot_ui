use egui::{Ui, ScrollArea, Align, Layout, FontId, TextEdit, Button, Vec2, Image};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::app::Icons;
use super::message_view;
use crate::providers::Provider;
use crate::ui::theme::DarkTheme;
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

    pub fn render(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, icons: &Icons, providers: &[Arc<dyn Provider + Send + Sync>], theme: &DarkTheme) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical(|ui| {
                let available_height = ui.available_height();
                let input_height = 80.0;
                let padding = 10.0;
                let message_height = available_height - input_height - padding * 2.0;

                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .max_height(message_height)
                    .show(ui, |ui| {
                        message_view::render_messages(ui, chat, &self.current_response, self.is_loading, theme);
                    });

                ui.add_space(padding);

                ui.horizontal(|ui| {
                    let input_field = TextEdit::multiline(&mut self.input)
                        .desired_width(ui.available_width() - 50.0)
                        .desired_rows(3)
                        .hint_text("Type your message here...")
                        .font(FontId::proportional(16.0))
                        .text_color(theme.override_text_color);

                    let response = ui.add_sized([ui.available_width() - 50.0, input_height], input_field);

                    ui.add_space(5.0);

                    let button_size = Vec2::new(40.0, input_height);
                    if ui.add_sized(button_size, Button::image(Image::new(&icons.send).fit_to_exact_size(Vec2::new(24.0, 24.0)))).clicked()
                        || (ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift))
                    {
                        if !self.input.trim().is_empty() {
                            chat.process_input(std::mem::take(&mut self.input));
                            self.is_loading = true;
                        }
                        response.request_focus();
                    }
                });

                ui.add_space(padding);
            });
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