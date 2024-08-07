use egui::{Ui, ScrollArea, TextEdit, Button, Label, Sense, RichText};
use crate::chat::Chat;

pub struct ChatbotUi {
    input: String,
}

impl ChatbotUi {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
    pub fn render(&mut self, ui: &mut Ui, chat: &mut Chat) {
        egui::TopBottomPanel::bottom("input_panel").show_inside(ui, |ui| {
            self.render_input(ui, chat);
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.render_messages(ui, chat);
        });
    }

    fn render_messages(&self, ui: &mut Ui, chat: &Chat) {
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            for message in chat.get_messages() {
                let mut text = if message.is_user() {
                    format!("You: {}", message.content())
                } else {
                    format!("Bot: {}", message.content())
                };
                
                let text_edit = TextEdit::multiline(&mut text)
                    .desired_width(f32::INFINITY)
                    .text_style(egui::TextStyle::Body)
                    .interactive(true)
                    .lock_focus(true)
                    .frame(false);
                
                let response = ui.add(text_edit);
                
                // Prevent actual edits
                if response.changed() {
                    text = if message.is_user() {
                        format!("You: {}", message.content())
                    } else {
                        format!("Bot: {}", message.content())
                    };
                }
    
                ui.add_space(5.0);
            }
        });
    }

    fn render_input(&mut self, ui: &mut Ui, chat: &mut Chat) {
        ui.horizontal(|ui| {
            let input_field = TextEdit::multiline(&mut self.input)
                .desired_rows(2)
                .desired_width(ui.available_width() - 60.0);
            let response = ui.add(input_field);
    
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Send").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if !self.input.trim().is_empty() {
                        chat.process_input(std::mem::take(&mut self.input));
                    }
                }
            });
        });
    }
}