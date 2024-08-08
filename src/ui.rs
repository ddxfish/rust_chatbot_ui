use egui::{
    Ui, ScrollArea, TextFormat, TextEdit, Button, Label, Sense, RichText, Vec2, 
    FontId, TextStyle, Color32, FontFamily, text::LayoutJob
};
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
                    //ui.add_space(4.0); // Add some space at the bottom of the window
                    self.render_input(ui, chat);
                });
            });
        });
    }

    fn render_messages(&self, ui: &mut Ui, chat: &Chat) {
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            for message in chat.get_messages() {
                let text = if message.is_user() {
                    format!("You: {}", message.content())
                } else {
                    format!("Bot: {}", message.content())
                };
                
                let (rect, _) = self.auto_sized_text_edit(ui, &text);
                ui.allocate_rect(rect, egui::Sense::hover());
            }
        });
    }

    fn auto_sized_text_edit(&self, ui: &mut Ui, text: &str) -> (egui::Rect, egui::Response) {
        let font_id = TextStyle::Body.resolve(ui.style());
        let row_height = ui.text_style_height(&TextStyle::Body);
        let width = ui.available_width();
        
        let job = LayoutJob::single_section(
            text.to_string(),
            TextFormat {
                font_id: font_id.clone(),
                color: ui.visuals().text_color(),
                ..Default::default()
            }
        );
        
        let galley = ui.painter().layout_job(job);
        let height = galley.rect.height().max(row_height);
        
        let mut text_owned = text.to_string();
        let mut text_edit = TextEdit::multiline(&mut text_owned)
            .desired_width(width)
            .font(font_id)
            .interactive(true)
            .lock_focus(true)
            .frame(false);
        
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());
        text_edit.show(ui);
        
        (rect, response)
    }
    fn render_input(&mut self, ui: &mut Ui, chat: &mut Chat) {
        let input_field = TextEdit::multiline(&mut self.input)
            .desired_rows(3)
            .hint_text("Type your message here...")
            //.text_style(egui::TextStyle::Monospace)
            .font(egui::FontId::proportional(14.0));
    
            let response = ui.add_sized(
                [ui.available_width(), ui.available_height()],
                input_field
            );
    
        let button_size = Vec2::new(25.0, 25.0);
        let button_pos = ui.min_rect().right_bottom() - button_size - Vec2::new(5.0, 35.0);
        
        if ui.put(egui::Rect::from_min_size(button_pos, button_size), Button::new("➤")).clicked()
           || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift)) {
            if !self.input.trim().is_empty() {
                chat.process_input(std::mem::take(&mut self.input));
            }
            response.request_focus();
        }
    }
}
    

