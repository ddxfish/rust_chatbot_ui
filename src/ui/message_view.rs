use egui::{Ui, ScrollArea, Align, FontId, TextFormat, text::LayoutJob};
use crate::chat::Chat;

pub fn render_messages(ui: &mut Ui, chat: &Chat) {
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
                ui.add_space(5.0);
            }
            
            if chat.is_processing() {
                ui.add(egui::Spinner::new());
            }
            
            let max_scroll = ui.max_rect().height() - ui.clip_rect().height();
            let current_scroll = ui.clip_rect().top() - ui.min_rect().top();
            scroll_to_bottom = (max_scroll - current_scroll).abs() < 1.0;
        });

    if scroll_to_bottom {
        ui.scroll_to_cursor(Some(Align::BOTTOM));
    }
}