use egui::{Ui, ScrollArea, Align, FontId, TextFormat, text::LayoutJob, FontFamily, Frame, Stroke, Rounding, Label, Layout, RichText};
use crate::chat::Chat;
use crate::ui::themes::Theme;
use crate::message::Message;
use std::collections::HashMap;
use crate::ui::syntax_highlighter::SyntaxHighlighter;

pub struct MessageView {
    cache: HashMap<usize, CachedMessage>,
    current_theme: String,
    syntax_highlighter: SyntaxHighlighter,
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
            current_theme: String::new(),
            syntax_highlighter: SyntaxHighlighter::new(),
        }
    }

    pub fn render_messages(&mut self, ui: &mut Ui, chat: &Chat, current_response: &str, is_loading: bool, theme: &Theme) {
        if self.current_theme != theme.name {
            self.cache.clear();
            self.current_theme = theme.name.clone();
        }

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
        let needs_update = if let Some(cached) = self.cache.get(&index) {
            cached.content != message.content() || cached.is_user != message.is_user() || cached.model.as_deref() != message.model()
        } else {
            true
        };

        if needs_update {
            let layout_job = self.create_layout_job(message, theme);
            self.cache.insert(index, CachedMessage {
                content: message.content().to_string(),
                is_user: message.is_user(),
                model: message.model().map(String::from),
                layout_job,
            });
        }

        let cached = self.cache.get(&index).unwrap();
        render_message_frame(ui, cached.is_user, &cached.layout_job, cached.model.as_deref(), theme);
    }

    fn render_current_response(&self, ui: &mut Ui, content: &str, model: String, theme: &Theme) {
        let message = Message::new(content.to_string(), false, Some(model));
        let layout_job = self.create_layout_job(&message, theme);
        render_message_frame(ui, false, &layout_job, message.model(), theme);
    }

    fn create_layout_job(&self, message: &Message, theme: &Theme) -> LayoutJob {
        self.syntax_highlighter.highlight_message(message, theme)
    }
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