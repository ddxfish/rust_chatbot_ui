use egui::{Ui, ScrollArea, TextEdit, Button, Label, Sense, RichText, Vec2};
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
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical(|ui| {
                let available_height = ui.available_height();
                let (message_height, input_height) = (available_height - 70.0, 70.0);
                
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .max_height(message_height)
                    .show(ui, |ui| {
                        self.render_messages(ui, chat);
                    });
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(4.0); // Add some space at the bottom of the window
                    self.render_input(ui, chat);
                });
            });
        });
    }

    fn render_messages(&self, ui: &mut Ui, chat: &Chat) {
        for message in chat.get_messages() {
            let text = if message.is_user() {
                format!("You: {}", message.content())
            } else {
                format!("Bot: {}", message.content())
            };
            
            ui.label(text);
            ui.add_space(2.0); // Reduced spacing
        }
    }

    fn render_input(&mut self, ui: &mut Ui, chat: &mut Chat) {
        let padding = 10.0; // Reduced padding to 5px
    
        ui.add_space(padding);
    
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = padding;
            
            let input_field = TextEdit::multiline(&mut self.input)
                .desired_rows(3)
                .hint_text("Type your message here...")
                .text_style(egui::TextStyle::Monospace)
                .font(egui::FontId::proportional(14.0))
                .frame(true);
    
            let response = ui.add_sized(
                [ui.available_width() - 90.0, 40.0], // Adjust width as needed
                input_field
            );
    
            if ui.add_sized([80.0, 40.0], Button::new("Send").rounding(5.0)).clicked() 
               || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift)) {
                if !self.input.trim().is_empty() {
                    chat.process_input(std::mem::take(&mut self.input));
                }
                response.request_focus();
            }
        });
    
        ui.add_space(padding);
    }
}