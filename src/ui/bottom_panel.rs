use egui::{Ui, ComboBox, Window, RichText, Layout, Align};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::providers::ProviderTrait;
use crate::ui::ChatbotUi;
use crate::ui::themes::Theme;
use rfd::FileDialog;
use std::path::Path;
use std::sync::Arc;

pub fn render(ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, chatbot_ui: &mut ChatbotUi, providers: &[Arc<dyn ProviderTrait + Send + Sync>], theme: &Theme) {
    static mut SHOW_CUSTOM_MODEL_POPUP: bool = false;
    static mut CUSTOM_MODEL_INPUT: String = String::new();

    ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
        ui.add_space(18.0);

        ui.horizontal(|ui| {
            let button_width = ui.available_width() / 2.0 - 10.0;

            if ui.add_sized([button_width, 30.0], egui::Button::new(RichText::new("Export").size(14.0).color(theme.button_text_color)).fill(theme.button_bg_color)).clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Text", &["txt"])
                    .set_directory("/")
                    .save_file() {
                    if let Err(e) = chat.export_chat(Path::new(&path)) {
                        eprintln!("Failed to export chat: {}", e);
                    }
                }
            }

            ui.add_space(10.0);

            if ui.add_sized([button_width, 30.0], egui::Button::new(RichText::new("Settings").size(14.0).color(theme.button_text_color)).fill(theme.button_bg_color)).clicked() {
                settings.show_settings = !settings.show_settings;
            }
        });

        ui.add_space(5.0);

        let dropdown_width = ui.available_width() * 0.99;

        ui.visuals_mut().widgets.inactive.bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().widgets.hovered.bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().widgets.active.bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().widgets.open.bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().selection.bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().widgets.noninteractive.bg_fill = theme.model_provider_dropdown_bg_color;

        

        ui.visuals_mut().widgets.inactive.weak_bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().widgets.hovered.weak_bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().widgets.active.weak_bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().widgets.open.weak_bg_fill = theme.model_provider_dropdown_bg_color;
        ui.visuals_mut().widgets.noninteractive.weak_bg_fill = theme.model_provider_dropdown_bg_color;

        ComboBox::from_id_source("model_combo")
        .selected_text(RichText::new(chatbot_ui.selected_model.as_str()).color(theme.model_provider_dropdown_text_color))
        .width(dropdown_width)
        .show_ui(ui, |ui| {
            if let Some(current_provider) = providers.iter().find(|p| p.name() == chatbot_ui.selected_provider) {
                for model in current_provider.models() {
                    if (model == "Other" && chatbot_ui.selected_provider == "GPT") || 
                       (model == "Other" && chatbot_ui.selected_provider == "Fireworks") {
                        if ui.selectable_label(false, RichText::new("Other").color(theme.model_provider_dropdown_text_color)).clicked() {
                            unsafe {
                                SHOW_CUSTOM_MODEL_POPUP = true;
                                CUSTOM_MODEL_INPUT.clear();
                            }
                        }
                    } else if ui.selectable_value(&mut chatbot_ui.selected_model, model.to_string(), RichText::new(model).color(theme.model_provider_dropdown_text_color)).clicked() {
                        chatbot_ui.selected_model = model.to_string();
                        chatbot_ui.model_changed = true;
                    }
                }
            }
        });

        ui.add_space(5.0);

        ComboBox::from_id_source("provider_combo")
            .selected_text(RichText::new(chatbot_ui.selected_provider.as_str()).color(theme.model_provider_dropdown_text_color))
            .width(dropdown_width)
            .show_ui(ui, |ui| {
                for provider in providers {
                    if ui.selectable_label(chatbot_ui.selected_provider == provider.name(), RichText::new(provider.name()).color(theme.model_provider_dropdown_text_color)).clicked() {
                        chatbot_ui.selected_provider = provider.name().to_string();
                        chatbot_ui.selected_model = provider.models()[0].to_string();
                        chatbot_ui.model_changed = true;
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
                        ui.label(RichText::new("Enter custom model name:").color(theme.settings_text_color));
                        ui.text_edit_singleline(&mut CUSTOM_MODEL_INPUT);
                    });
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(RichText::new("Cancel").color(theme.settings_button_text_color)).fill(theme.settings_button_bg_color)).clicked() {
                            SHOW_CUSTOM_MODEL_POPUP = false;
                        }
                        if ui.add(egui::Button::new(RichText::new("OK").color(theme.settings_button_text_color)).fill(theme.settings_button_bg_color)).clicked() {
                            chatbot_ui.selected_model = if chatbot_ui.selected_provider == "Fireworks" {
                                format!("accounts/fireworks/models/{}", CUSTOM_MODEL_INPUT.clone())
                            } else {
                                CUSTOM_MODEL_INPUT.clone()
                            };
                            chatbot_ui.model_changed = true;
                            SHOW_CUSTOM_MODEL_POPUP = false;
                        }
                    });
                });
        }
    }
}