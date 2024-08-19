use egui::{Ui, ScrollArea, Align, FontId, TextFormat, text::LayoutJob, FontFamily, Frame, Stroke, Rounding};
use crate::chat::Chat;
use crate::ui::theme::DarkTheme;

pub fn render_messages(ui: &mut Ui, chat: &Chat, current_response: &str, is_loading: bool, theme: &DarkTheme) {
    let mut scroll_to_bottom = false;
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for message in chat.get_messages() {
                render_message(ui, message.is_user(), message.content(), theme);
            }

            if !current_response.is_empty() {
                render_message(ui, false, current_response, theme);
            }

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

fn render_message(ui: &mut Ui, is_user: bool, content: &str, theme: &DarkTheme) {
    let (border_color, background_color) = if is_user {
        (theme.user_message_border, theme.user_message_bg)
    } else {
        (theme.bot_message_border, theme.bot_message_bg)
    };

    let frame = Frame::none()
        .fill(background_color)
        .stroke(Stroke::new(1.0, border_color))
        .rounding(Rounding::same(5.0))
        .outer_margin(10.0)
        .inner_margin(10.0);

    frame.show(ui, |ui| {
        let mut job = LayoutJob::default();
        let prefix = if is_user { "You: " } else { "Bot: " };
        let text = format!("{}{}\n", prefix, content);
        
        job.append(
            &text,
            0.0,
            TextFormat {
                font_id: FontId::new(18.0, FontFamily::Proportional),
                color: theme.message_text_color,
                ..Default::default()
            },
        );

        ui.label(job);
    });
}