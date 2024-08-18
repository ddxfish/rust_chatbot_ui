use egui::{Ui, ComboBox, Window, TextEdit, RichText, Layout};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::providers::Provider;
use crate::ui::ChatbotUi;
use rfd::FileDialog;
use std::path::Path;
use std::sync::Arc;

pub fn render(ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, chatbot_ui: &mut ChatbotUi, providers: &[Arc<dyn Provider + Send + Sync>]) {
    static mut SHOW_CUSTOM_MODEL_POPUP: bool = false;
    static mut CUSTOM_MODEL_INPUT: String = String::new();

    ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
        ComboBox::from_id_source("provider_combo")
            .selected_text(chatbot_ui.selected_provider.as_str())
            .show_ui(ui, |ui| {
                for provider in providers {
                    if ui.selectable_label(chatbot_ui.selected_provider == provider.name(), provider.name()).clicked() {
                        chatbot_ui.selected_provider = provider.name().to_string();
                        chatbot_ui.selected_model = provider.models()[0].to_string();
                    }
                }
            });

        ui.add_space(5.0);

        if let Some(current_provider) = providers.iter().find(|p| p.name() == chatbot_ui.selected_provider) {
            ComboBox::from_id_source("model_combo")
                .selected_text(chatbot_ui.selected_model.as_str())
                .show_ui(ui, |ui| {
                    for model in current_provider.models() {
                        if ui.selectable_value(&mut chatbot_ui.selected_model, model.to_string(), model).clicked() {
                            chatbot_ui.selected_model = model.to_string();
                        }
                    }
                    if ui.selectable_label(false, "Other").clicked() {
                        unsafe {
                            SHOW_CUSTOM_MODEL_POPUP = true;
                            CUSTOM_MODEL_INPUT.clear();
                        }
                    }
                });
        }

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            if ui.button(RichText::new("Export").size(14.0)).clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Text", &["txt"])
                    .set_directory("/")
                    .save_file() {
                    if let Err(e) = chat.export_chat(Path::new(&path)) {
                        eprintln!("Failed to export chat: {}", e);
                    }
                }
            }

            if ui.button(RichText::new("Settings").size(14.0)).clicked() {
                settings.toggle_settings();
            }
        });
    });

    unsafe {
        if SHOW_CUSTOM_MODEL_POPUP {
            Window::new("Custom Model")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Enter custom model name:");
                        ui.text_edit_singleline(&mut CUSTOM_MODEL_INPUT);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            SHOW_CUSTOM_MODEL_POPUP = false;
                        }
                        if ui.button("OK").clicked() {
                            chatbot_ui.selected_model = CUSTOM_MODEL_INPUT.clone();
                            SHOW_CUSTOM_MODEL_POPUP = false;
                        }
                    });
                });
        }
    }
}