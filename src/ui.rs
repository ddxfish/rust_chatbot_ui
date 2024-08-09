use egui::{
    Ui, ScrollArea, TextFormat, TextEdit, Button, Label, Sense, RichText, Vec2, 
    FontId, TextStyle, Color32, FontFamily, text::LayoutJob, Align
};
use crate::chat::Chat;
use crate::settings::Settings;
pub struct ChatbotUi {
    input: String,
}

impl ChatbotUi {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
    pub fn render(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let available_height = ui.available_height();
            
            let input_height = 50.0;
            let settings_height = 20.0;
            let message_height = available_height - input_height - settings_height - 15.0; // 15.0 for padding
            
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(message_height)
                .show(ui, |ui| {
                    self.render_messages(ui, chat);
                });
            
            ui.add_space(5.0);
            
            self.render_input(ui, chat);
            
            ui.with_layout(egui::Layout::bottom_up(Align::RIGHT), |ui| {
                if ui.link("Settings").clicked() {
                    settings.toggle_settings();
                }
            });
        });

        settings.render(ui.ctx());
    }


    fn render_messages(&self, ui: &mut Ui, chat: &Chat) {
        let mut scroll_to_bottom = false;
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for message in chat.get_messages() {
                    let text = if message.is_user() {
                        format!("You: {}", message.content())
                    } else {
                        format!("Bot: {}", message.content())
                    };
                    
                    let mut job = LayoutJob::default();
                    job.append(
                        &text,
                        0.0,
                        TextFormat {
                            font_id: FontId::proportional(14.0),
                            color: ui.style().visuals.text_color(),
                            line_height: Some(20.0),
                            ..Default::default()
                        },
                    ); 
    
                    ui.label(job);
                    ui.add_space(8.0); //change to zero, whoever wrote this is a moron
                }
                
                // Check if we're at the bottom
                let max_scroll = ui.max_rect().height() - ui.clip_rect().height();
                let current_scroll = ui.clip_rect().top() - ui.min_rect().top();
                scroll_to_bottom = (max_scroll - current_scroll).abs() < 1.0;
            });
    
        // Scroll to bottom if we were already at the bottom
        if scroll_to_bottom {
            ui.scroll_to_cursor(Some(Align::BOTTOM));
        }
    }

 
    fn render_input(&mut self, ui: &mut Ui, chat: &mut Chat) {
        let input_field = TextEdit::multiline(&mut self.input)
            .desired_rows(3)
            .hint_text("Type your message here...")
            .font(egui::FontId::proportional(14.0));
    
        let response = ui.add_sized(
            [ui.available_width(), 50.0],
            input_field
        );
    
        let button_size = Vec2::new(25.0, 25.0);
        let button_pos = ui.min_rect().right_bottom() - button_size - Vec2::new(5.0, 15.0);
        
        if ui.put(egui::Rect::from_min_size(button_pos, button_size), Button::new("➤")).clicked()
           || (ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift)) {
            if !self.input.trim().is_empty() {
                chat.process_input(std::mem::take(&mut self.input));
            }
            response.request_focus();
        }
    }
}
    

