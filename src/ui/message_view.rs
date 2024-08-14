use egui::{Ui, ScrollArea, Align, FontId, TextFormat, text::LayoutJob};
use crate::chat::Chat;

pub fn render_messages(ui: &mut Ui, chat: &Chat, current_response: &str, is_loading: bool) {
    let mut scroll_to_bottom = false;
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            let mut job = LayoutJob::default();

            for message in chat.get_messages() {
                add_message_to_job(&mut job, message.is_user(), message.content(), ui);
            }

            // Add the current (streaming) response
            if !current_response.is_empty() {
                add_message_to_job(&mut job, false, current_response, ui);
            }

            ui.label(job);

            if is_loading {
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

fn add_message_to_job(job: &mut LayoutJob, is_user: bool, content: &str, ui: &Ui) {
    let prefix = if is_user { "You: " } else { "Bot: " };
    let text = format!("{}{}\n\n", prefix, content);
    
    job.append(
        &text,
        0.0,
        TextFormat {
            font_id: FontId::proportional(14.0),
            color: ui.style().visuals.text_color(),
            ..Default::default()
        },
    );
}