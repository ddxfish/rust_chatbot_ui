use egui::{Ui, ScrollArea, Align, Frame, Stroke, Rounding, Label, Layout, RichText};
use crate::chat::Chat;
use crate::ui::themes::Theme;
use crate::message::Message;
use std::collections::HashMap;
use crate::ui::syntax_highlighter::{SyntaxHighlighter, HighlightedBlock};

pub struct MessageView {
    syntax_highlighter: SyntaxHighlighter,
    message_cache: HashMap<String, Vec<HighlightedBlock>>,
}

impl MessageView {
    pub fn new() -> Self {
        Self {
            syntax_highlighter: SyntaxHighlighter::new(),
            message_cache: HashMap::new(),
        }
    }

    pub fn render_messages(&mut self, ui: &mut Ui, chat: &Chat, current_response: &str, is_loading: bool, theme: &Theme) {
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                let messages = chat.get_messages();
                for message in messages.iter() {
                    self.render_message(ui, message, theme);
                }

                if !current_response.is_empty() {
                    self.render_current_response(ui, current_response, chat.get_current_model(), theme);
                }

                if is_loading {
                    ui.add(egui::Spinner::new());
                }
            });
    }

    fn render_message(&mut self, ui: &mut Ui, message: &Message, theme: &Theme) {
        let cache_key = format!("{}-{}", message.content(), message.is_user());
        let highlighted_blocks = self.message_cache.entry(cache_key.clone())
            .or_insert_with(|| self.syntax_highlighter.highlight_message(
                message.content(),
                message.is_user(),
                theme,
                matches!(theme.name.as_str(), "Light" | "Olive and Tan"),
                false
            ));
        MessageView::render_message_frame(ui, message.is_user(), highlighted_blocks, message.model(), theme);
    }

    fn render_current_response(&self, ui: &mut Ui, content: &str, model: String, theme: &Theme) {
        let message = Message::new(content.to_string(), false, Some(model));
        let highlighted_blocks = self.syntax_highlighter.highlight_message(
            message.content(),
            message.is_user(),
            theme,
            matches!(theme.name.as_str(), "Light" | "Olive and Tan"),
            true
        );
        MessageView::render_message_frame(ui, message.is_user(), &highlighted_blocks, message.model(), theme);
    }

    fn render_message_frame(ui: &mut Ui, is_user: bool, highlighted_blocks: &[HighlightedBlock], model: Option<&str>, theme: &Theme) {
        let (border_color, background_color, name_color) = if is_user {
            (theme.user_message_border, theme.user_message_bg, theme.user_name_text_color)
        } else {
            (theme.bot_message_border, theme.bot_message_bg, theme.bot_name_text_color)
        };

        Frame::none()
            .fill(background_color)
            .stroke(Stroke::new(1.0, border_color))
            .rounding(Rounding::same(5.0))
            .outer_margin(10.0)
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    let prefix = if is_user { 
                        RichText::new("You:").strong().size(18.0).color(name_color)
                    } else { 
                        RichText::new(format!("{}:", model.unwrap_or("Bot"))).strong().size(18.0).color(name_color)
                    };
                    ui.label(prefix);

                    for block in highlighted_blocks {
                        match block {
                            HighlightedBlock::Text(job) => {
                                ui.add(Label::new(job.clone()).wrap());
                            },
                            HighlightedBlock::Code { language, job } => {
                                ui.add_space(5.0);
                                Frame::none()
                                    .fill(theme.code_block_bg)
                                    .stroke(Stroke::new(1.0, theme.code_block_border))
                                    .rounding(Rounding::same(5.0))
                                    .outer_margin(0.0)
                                    .inner_margin(18.0)
                                    .show(ui, |ui| {
                                        ui.set_max_width(ui.available_width() * 0.99);
                                        if !language.is_empty() {
                                            ui.label(RichText::new(language).small().color(theme.code_block_language_color));
                                        }
                                        ui.add(Label::new(job.clone()).wrap());
                                    });
                                ui.add_space(5.0);
                            }
                        }
                    }
                });
            });

        ui.add_space(10.0);
    }

    pub fn clear_syntax_cache(&mut self) {
        self.syntax_highlighter.clear_cache();
        self.message_cache.clear();
    }
    pub fn clear_cache(&mut self) {
        self.message_cache.clear();
        self.syntax_highlighter.clear_cache();  // If SyntaxHighlighter has its own cache
    }
}