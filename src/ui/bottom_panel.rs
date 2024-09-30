use egui::{Ui, ComboBox, RichText, Layout, Align};
use crate::chat::Chat;
use crate::settings::Settings;
use crate::providers::ProviderTrait;
use crate::ui::ChatbotUi;
use crate::ui::themes::Theme;
use crate::app::ProfileType;
use rfd::FileDialog;
use std::path::Path;
use std::sync::Arc;

pub fn render(ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, chatbot_ui: &mut ChatbotUi, providers: &[Arc<dyn ProviderTrait + Send + Sync>], theme: &Theme, current_profile: &mut ProfileType) {
    ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
        ui.add_space(18.0);
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

        ComboBox::from_id_source("model_combo")
        .selected_text(RichText::new(chatbot_ui.selected_model.as_str()).color(theme.model_provider_dropdown_text_color))
        .width(dropdown_width)
        .show_ui(ui, |ui| {
            if let Some(current_provider) = providers.iter().find(|p| p.name() == chatbot_ui.selected_provider) {
                for model in current_provider.models() {
                    if ui.selectable_value(&mut chatbot_ui.selected_model, model.to_string(), RichText::new(model).color(theme.model_provider_dropdown_text_color)).clicked() {
                        chatbot_ui.selected_model = model.to_string();
                        chatbot_ui.model_changed = true;
                    }
                }
                if ui.selectable_value(&mut chatbot_ui.selected_model, "Other".to_string(), RichText::new("Other").color(theme.model_provider_dropdown_text_color)).clicked() {
                    chatbot_ui.show_custom_model_popup = true;
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

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            let profile_frame = egui::Frame::none()
                .fill(theme.model_provider_dropdown_bg_color)
                .rounding(5.0)
                .stroke(egui::Stroke::new(1.0, theme.model_provider_dropdown_text_color));

            profile_frame.show(ui, |ui| {
                ui.set_width(dropdown_width);
                ui.horizontal(|ui| {
                    for profile in [ProfileType::Coder, ProfileType::Normal, ProfileType::Creative].iter() {
                        if ui.selectable_label(*current_profile == *profile, format!("{:?}", profile)).clicked() {
                            *current_profile = *profile;
                            if let Some(provider) = providers.iter().find(|p| p.name() == chatbot_ui.selected_provider) {
                                provider.update_profile(*current_profile);
                            }
                            chat.update_profile(*current_profile);
                        }
                    }
                });
            });
        });
    });

    if chatbot_ui.show_custom_model_popup {
        egui::Window::new("Custom Model")
            .show(ui.ctx(), |ui| {
                ui.text_edit_singleline(&mut chatbot_ui.custom_model_name);
                if ui.button("Confirm").clicked() {
                    chatbot_ui.selected_model = chatbot_ui.custom_model_name.clone();
                    chatbot_ui.show_custom_model_popup = false;
                    chatbot_ui.model_changed = true;
                }
            });
    }
}