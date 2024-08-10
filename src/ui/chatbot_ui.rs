use egui::{Ui, ScrollArea, Align, Layout, ComboBox, Window};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::app::Icons;
use crate::providers::{Provider, get_providers};
use super::{message_view, input_area, bottom_panel};

pub struct ModelSelector {
    selected_model: String,
    custom_model_input: String,
    show_custom_model_popup: bool,
}

impl ModelSelector {
    fn new() -> Self {
        Self {
            selected_model: String::new(),
            custom_model_input: String::new(),
            show_custom_model_popup: false,
        }
    }

    fn render(&mut self, ui: &mut Ui, models: &[&str]) {
        ComboBox::from_label("Model")
            .selected_text(&self.selected_model)
            .show_ui(ui, |ui| {
                for &model in models {
                    ui.selectable_value(&mut self.selected_model, model.to_string(), model);
                }
                if ui.selectable_label(false, "Other").clicked() {
                    self.show_custom_model_popup = true;
                }
            });

        if self.show_custom_model_popup {
            Window::new("Custom Model")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Enter custom model name:");
                        ui.text_edit_singleline(&mut self.custom_model_input);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_custom_model_popup = false;
                        }
                        if ui.button("OK").clicked() {
                            self.selected_model = self.custom_model_input.clone();
                            self.show_custom_model_popup = false;
                        }
                    });
                });
        }
    }
}

pub struct ChatbotUi {
    input: String,
    selected_provider: String,
    model_selector: ModelSelector,
}

impl ChatbotUi {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            selected_provider: String::new(),
            model_selector: ModelSelector::new(),
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
                    message_view::render_messages(ui, chat);
                });
            
            input_area::render_input(ui, chat, icons, &mut self.input);
            
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                if bottom_padding > 0.0 {
                    ui.allocate_space(egui::vec2(ui.available_width(), bottom_padding));
                }
                
                bottom_panel::render(ui, chat, settings, &mut self.selected_provider, &mut self.model_selector.selected_model);
            });
        });
    
        settings.render(ui.ctx(), icons);
    }
}