use egui::{Ui, ComboBox, Layout, Align, Window, TextEdit, Id};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::providers::Provider;
use rfd::FileDialog;
use std::path::Path;
use std::sync::Arc;

pub fn render(ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, selected_provider: &mut String, selected_model: &mut String, providers: &[Arc<dyn Provider + Send + Sync>]) {
    static mut SHOW_CUSTOM_MODEL_POPUP: bool = false;
    static mut CUSTOM_MODEL_INPUT: String = String::new();

    ui.horizontal(|ui| {
        ui.set_min_height(30.0);
        ui.add_space(2.0);
        
        ComboBox::from_id_source("provider_combo")
            .selected_text(selected_provider.as_str())
            .show_ui(ui, |ui| {
                for provider in providers {
                    if ui.selectable_label(selected_provider == provider.name(), provider.name()).clicked() {
                        *selected_provider = provider.name().to_string();
                        *selected_model = provider.models()[0].to_string();
                    }
                }
            });

        ui.add_space(10.0);

        if let Some(current_provider) = providers.iter().find(|p| p.name() == *selected_provider) {
            ComboBox::from_id_source("model_combo")
                .selected_text(selected_model.as_str())
                .show_ui(ui, |ui| {
                    for model in current_provider.models() {
                        if ui.selectable_value(selected_model, model.to_string(), model).clicked() {
                            *selected_model = model.to_string();
                        }
                    }
                    if ui.selectable_label(false, "Other").clicked() {
                        unsafe {
                            SHOW_CUSTOM_MODEL_POPUP = true;
                            CUSTOM_MODEL_INPUT.clear();
                        }
                    }
                });
        } else {
            ui.label("Select a provider first");
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.link("Settings").clicked() {
                settings.toggle_settings();
            }
            if ui.button("Export").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Text", &["txt"])
                    .set_directory("/")
                    .save_file() {
                    if let Err(e) = chat.export_chat(Path::new(&path)) {
                        eprintln!("Failed to export chat: {}", e);
                    }
                }
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
                            *selected_model = CUSTOM_MODEL_INPUT.clone();
                            SHOW_CUSTOM_MODEL_POPUP = false;
                        }
                    });
                });
        }
    }
}