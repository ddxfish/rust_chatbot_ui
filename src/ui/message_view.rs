use egui::{Ui, ScrollArea, Align, FontId, TextFormat, text::LayoutJob, FontFamily, Frame, Stroke, Rounding, Label, Layout, RichText};
use crate::chat::Chat;
use crate::ui::themes::Theme;

pub fn render_messages(ui: &mut Ui, chat: &Chat, current_response: &str, is_loading: bool, theme: &Theme) {
    let mut scroll_to_bottom = false;
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            let messages = chat.get_messages();
            for (index, message) in messages.iter().enumerate() {
                render_message(ui, message.is_user(), message.content(), message.model(), theme);
                if index == messages.len() - 1 && !message.is_user() && !current_response.is_empty() {
                    continue;
                }
            }

            if !current_response.is_empty() {
                render_message(ui, false, current_response, Some(&chat.get_current_model()), theme);
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

fn render_message(ui: &mut Ui, is_user: bool, content: &str, model: Option<&str>, theme: &Theme) {
    let (border_color, background_color, text_color, name_color) = if is_user {
        (theme.user_message_border, theme.user_message_bg, theme.user_text_color, theme.user_name_text_color)
    } else {
        (theme.bot_message_border, theme.bot_message_bg, theme.bot_text_color, theme.bot_name_text_color)
    };

    let frame = Frame::none()
        .fill(background_color)
        .stroke(Stroke::new(1.0, border_color))
        .rounding(Rounding::same(5.0))
        .outer_margin(10.0)
        .inner_margin(10.0);

    frame.show(ui, |ui| {
        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            let prefix = if is_user { 
                RichText::new("You:").strong().size(18.0).color(name_color)
            } else { 
                RichText::new(format!("{}:", model.unwrap_or("Bot"))).strong().size(18.0).color(name_color)
            };
            ui.label(prefix);

            //ui.add_space(5.0);

            let mut job = LayoutJob::default();
            job.append(
                content,
                0.0,
                TextFormat {
                    font_id: FontId::new(16.0, FontFamily::Proportional),
                    color: text_color,
                    ..Default::default()
                },
            );

            ui.add(Label::new(job).wrap());
        });
    });

    ui.add_space(10.0);
}