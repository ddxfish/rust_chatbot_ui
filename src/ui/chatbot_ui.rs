use egui::{Ui, ScrollArea, FontId, TextEdit, Button, Vec2, Image, Layout, Align};
use crate::chat::Chat;
use crate::settings;
use crate::settings::Settings;
use crate::app::Icons;
use super::MessageView;
use crate::providers::Provider;
use crate::ui::themes::Theme;
use std::sync::Arc;

pub struct ChatbotUi {
    pub input: String,
    pub selected_provider: String,
    pub selected_model: String,
    pub is_loading: bool,
    pub current_response: String,
    pub model_changed: bool,
    message_view: MessageView,
}

impl ChatbotUi {
    pub fn new(initial_provider: String, initial_model: String) -> Self {
        Self {
            input: String::new(),
            selected_provider: initial_provider,
            selected_model: initial_model,
            is_loading: false,
            current_response: String::new(),
            model_changed: false,
            message_view: MessageView::new(),
        }
    }

    pub fn render(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, icons: &Icons, providers: &[Arc<dyn Provider + Send + Sync>], theme: &Theme) {
        if self.model_changed {
            if let Some(provider) = providers.iter().find(|p| p.name() == self.selected_provider) {
                chat.update_provider(Arc::clone(provider));
                chat.set_current_model(&self.selected_model);
                println!("Debug: Provider updated to {} with model {}", self.selected_provider, self.selected_model);
            }
            self.model_changed = false;
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical(|ui| {
                let available_height = ui.available_height();
                let input_height = 80.0;
                let padding = 0.0;
                let message_height = available_height - input_height - padding * 2.0;

                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .max_height(message_height)
                    .show(ui, |ui| {
                        self.message_view.render_messages(ui, chat, &self.current_response, self.is_loading, theme);
                    });

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(false), |ui| {
                        let input_width = ui.available_width() - 50.0;

                        ScrollArea::vertical()
                            .max_height(input_height)
                            .min_scrolled_height(80.0)
                            .show(ui, |ui| {
                                let input_field = TextEdit::multiline(&mut self.input)
                                    .desired_width(input_width)
                                    .desired_rows(3)
                                    .hint_text("Type your message here...")
                                    .font(FontId::proportional(16.0))
                                    .text_color(theme.input_text_color);

                                ui.add_sized([input_width, input_height], input_field);
                            });

                        let button_size = Vec2::new(40.0, input_height);
                        let icon = if self.is_loading { &icons.stop } else { &icons.send };
                        if ui.add_sized(button_size, Button::image(Image::new(icon).fit_to_exact_size(Vec2::new(24.0, 24.0))).fill(theme.button_bg_color)).clicked()
                            || (!self.is_loading && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift))
                        {
                            if self.is_loading {
                                chat.stop_processing();
                                self.is_loading = false;
                                self.current_response.clear();
                            } else if !self.input.trim().is_empty() {
                                println!("Debug: Processing input with model: {}", self.selected_model);
                                chat.process_input(std::mem::take(&mut self.input), &self.selected_model);
                                self.is_loading = true;
                                self.current_response.clear();
                            }
                        }
                    });
                });
            });
        });

        settings::render(settings, ui.ctx(), icons);

        if chat.is_processing() {
            self.is_loading = true;
        }

        while let Some((chunk, is_complete)) = chat.check_ui_updates() {
            if is_complete {
                chat.add_message(chunk, false);
                self.current_response.clear();
                self.is_loading = false;
            } else {
                self.current_response.push_str(&chunk);
            }
            ui.ctx().request_repaint();
        }

        if let Some(error) = chat.check_error_updates() {
            chat.add_message(error, false);
            self.is_loading = false;
            self.current_response.clear();
            ui.ctx().request_repaint();
        }

        if let Some(new_name) = chat.check_name_updates() {
            if let Err(e) = chat.rename_current_chat(&new_name) {
                eprintln!("Error: Failed to rename chat: {}", e);
            }
        }
    }
}