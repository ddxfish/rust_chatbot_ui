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
        ui.vertical(|ui| {
            self.render_messages(ui, chat);
            ui.add_space(10.0);
            self.render_input(ui, chat);
        });
    }

    fn render_messages(&self, ui: &mut Ui, chat: &Chat) {
        ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            for message in chat.get_messages() {
                let mut text = if message.is_user() {
                    format!("You: {}", message.content())
                } else {
                    format!("Bot: {}", message.content())
                };
                
                let text_edit = TextEdit::multiline(&mut text)
                    .desired_width(f32::INFINITY)
                    //.text_style(egui::TextStyle::Body)
                    .interactive(true)  // Make it interactive
                    .lock_focus(true);   // Prevent focus loss on click
                    //.selection_color(egui::Color32::from_rgba_premultiplied(100, 100, 100, 100));
                
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
            let text_edit = TextEdit::singleline(&mut self.input)
                .desired_width(ui.available_width() - 60.0);
            let text_edit_response = ui.add(text_edit);

            if ui.add(Button::new("Send")).clicked() || text_edit_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if !self.input.is_empty() {
                    chat.process_input(self.input.clone());
                    self.input.clear();
                }
            }
        });
    }
}