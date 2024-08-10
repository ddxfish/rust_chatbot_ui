use egui::{Ui, ComboBox, Layout, Align};
use crate::chat::Chat;
use crate::settings::Settings;
use rfd::FileDialog;
use super::theme;

pub fn render(ui: &mut Ui, chat: &mut Chat, settings: &mut Settings, selected_provider: &mut String, selected_model: &mut String) {
    ui.horizontal(|ui| {
        ui.set_min_height(30.0);
        ui.add_space(2.0);
        ComboBox::new("ai_provider", "")
        .selected_text(selected_provider.as_str())
            .show_ui(ui, |ui| {
                ui.selectable_value(selected_provider, "Provider 1".to_string(), "Provider 1");
                ui.selectable_value(selected_provider, "Provider 2".to_string(), "Provider 2");
                ui.selectable_value(selected_provider, "Provider 3".to_string(), "Provider 3");
            });

        ui.add_space(10.0);

        ComboBox::new("ai_model", "")
        .selected_text(selected_model.as_str())            .show_ui(ui, |ui| {
                ui.selectable_value(selected_model, "Model 1".to_string(), "Model 1");
                ui.selectable_value(selected_model, "Model 2".to_string(), "Model 2");
                ui.selectable_value(selected_model, "Model 3".to_string(), "Model 3");
            });

        if ui.button("Light").clicked() {
            ui.ctx().set_visuals(theme::custom_light_theme());
        }

        if ui.button("Dark").clicked() {
            ui.ctx().set_visuals(egui::Visuals::dark());
        }

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

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.link("Settings").clicked() {
                settings.toggle_settings();
            }
        });
    });
}