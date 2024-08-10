use egui::{
    Ui, ScrollArea, TextFormat, TextEdit, Button, Label, Sense, RichText, Vec2, 
    FontId, TextStyle, Color32, FontFamily, text::LayoutJob, Align, Layout, ComboBox, Image
};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::app::Icons;
use std::path::PathBuf;
use rfd::FileDialog;

pub struct ChatbotUi {
    input: String,
    selected_provider: String,
    selected_model: String,
}

impl ChatbotUi {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            selected_provider: "Provider 1".to_string(),
            selected_model: "Model 1".to_string(),
        }
    }
    pub fn render(&mut self, ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, icons: &Icons) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let available_height = ui.available_height();
            
            let bottom_row_height = 30.0;
            let input_height = 45.0;
            let bottom_padding = 0.0;
            let message_height = available_height - input_height - bottom_row_height - bottom_padding;
            
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(message_height)
                .show(ui, |ui| {
                    self.render_messages(ui, chat);
                });
            
            self.render_input(ui, chat, icons);
            
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                if bottom_padding > 0.0 {
                    ui.allocate_space(egui::vec2(ui.available_width(), bottom_padding));
                }
                
                ui.horizontal(|ui| {
                    ui.set_min_height(bottom_row_height);
                    ui.add_space(2.0);
                    ComboBox::new("ai_provider", "")
                        .selected_text(&self.selected_provider)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_provider, "Provider 1".to_string(), "Provider 1");
                            ui.selectable_value(&mut self.selected_provider, "Provider 2".to_string(), "Provider 2");
                            ui.selectable_value(&mut self.selected_provider, "Provider 3".to_string(), "Provider 3");
                        });
    
                    ui.add_space(10.0);
    
                    ComboBox::new("ai_model", "")
                        .selected_text(&self.selected_model)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_model, "Model 1".to_string(), "Model 1");
                            ui.selectable_value(&mut self.selected_model, "Model 2".to_string(), "Model 2");
                            ui.selectable_value(&mut self.selected_model, "Model 3".to_string(), "Model 3");
                        });
    
                    // Light mode button
                    if ui.button("Light").clicked() {
                        ui.ctx().set_visuals(egui::Visuals::light());
                    }

                    // Dark mode button
                    if ui.button("Dark").clicked() {
                        ui.ctx().set_visuals(egui::Visuals::dark());
                    }

                    // Export chat button
                    if ui.button("Export Chat").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Text", &["txt"])
                            .set_directory("/")
                            .save_file() {
                            if let Err(e) = chat.export_chat(&path) {
                                eprintln!("Failed to export chat: {}", e);
                            }
                        }
                    }

                    // Push settings link to the right
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.link("Settings").clicked() {
                            settings.toggle_settings();
                        }
                    });
                });
            });
        });
    
        settings.render(ui.ctx(), icons);
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
                    ui.add_space(0.0);
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

    fn render_input(&mut self, ui: &mut Ui, chat: &mut Chat, icons: &Icons) {
        let input_field = TextEdit::multiline(&mut self.input)
            .desired_rows(3)
            .hint_text("Type your message here...")
            .font(egui::FontId::proportional(14.0));
    
        let response = ui.add_sized(
            [ui.available_width(), 50.0],
            input_field
        );
    
        let button_size = Vec2::new(25.0, 25.0);
        let button_pos = ui.min_rect().right_bottom() - button_size - Vec2::new(5.0, 42.0);
        
        if ui.put(egui::Rect::from_min_size(button_pos, button_size), Button::image(Image::new(&icons.send).fit_to_exact_size(button_size))).clicked()
           || (ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift)) {
            if !self.input.trim().is_empty() {
                chat.process_input(std::mem::take(&mut self.input));
            }
            response.request_focus();
        }
    }
}