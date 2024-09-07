use egui::{Ui, ScrollArea, Align, FontId, TextFormat, text::LayoutJob, FontFamily, Frame, Stroke, Rounding, Label, Layout, RichText};
use crate::chat::Chat;
use crate::ui::themes::Theme;
use crate::message::Message;
use std::collections::HashMap;

pub struct MessageView {
    cache: HashMap<usize, CachedMessage>,
}

struct CachedMessage {
    content: String,
    is_user: bool,
    model: Option<String>,
    layout_job: LayoutJob,
}

impl MessageView {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn render_messages(&mut self, ui: &mut Ui, chat: &Chat, current_response: &str, is_loading: bool, theme: &Theme) {
        let mut scroll_to_bottom = false;
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                let messages = chat.get_messages();
                for (index, message) in messages.iter().enumerate() {
                    self.render_message(ui, index, message, theme);
                    if index == messages.len() - 1 && !message.is_user() && !current_response.is_empty() {
                        continue;
                    }
                }

                if !current_response.is_empty() {
                    self.render_current_response(ui, current_response, chat.get_current_model(), theme);
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

    fn render_message(&mut self, ui: &mut Ui, index: usize, message: &Message, theme: &Theme) {
        let cached = self.cache.entry(index).or_insert_with(|| {
            CachedMessage {
                content: message.content().to_string(),
                is_user: message.is_user(),
                model: message.model().map(String::from),
                layout_job: create_layout_job(message, theme),
            }
        });

        if cached.content != message.content() || cached.is_user != message.is_user() || cached.model.as_deref() != message.model() {
            cached.content = message.content().to_string();
            cached.is_user = message.is_user();
            cached.model = message.model().map(String::from);
            cached.layout_job = create_layout_job(message, theme);
        }

        render_message_frame(ui, message.is_user(), &cached.layout_job, message.model(), theme);
    }

    fn render_current_response(&self, ui: &mut Ui, content: &str, model: String, theme: &Theme) {
        let message = Message::new(content.to_string(), false, Some(model));
        let layout_job = create_layout_job(&message, theme);
        render_message_frame(ui, false, &layout_job, message.model(), theme);
    }
}

fn create_layout_job(message: &Message, theme: &Theme) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.append(
        message.content(),
        0.0,
        TextFormat {
            font_id: FontId::new(16.0, FontFamily::Proportional),
            color: if message.is_user() { theme.user_text_color } else { theme.bot_text_color },
            ..Default::default()
        },
    );
    job
}

fn render_message_frame(ui: &mut Ui, is_user: bool, layout_job: &LayoutJob, model: Option<&str>, theme: &Theme) {
    let (border_color, background_color, name_color) = if is_user {
        (theme.user_message_border, theme.user_message_bg, theme.user_name_text_color)
    } else {
        (theme.bot_message_border, theme.bot_message_bg, theme.bot_name_text_color)
    };

    let frame = Frame::none()
        .fill(background_color)
        .stroke(Stroke::new(1.0, border_color))
        .rounding(Rounding::same(5.0))
        .outer_margin(10.0)
        .inner_margin(10.0);

    frame.show(ui, |ui| {
        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            let prefix = if is_user { 
                RichText::new("You:").strong().size(18.0).color(name_color)
            } else { 
                RichText::new(format!("{}:", model.unwrap_or("Bot"))).strong().size(18.0).color(name_color)
            };
            ui.label(prefix);

            ui.add(Label::new(layout_job.clone()).wrap());
        });
    });

    ui.add_space(10.0);
}