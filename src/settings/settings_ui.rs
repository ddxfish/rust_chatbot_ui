use eframe::egui::{self, Window, RichText, DragValue};
use crate::app::Icons;
use super::Settings;
use super::settings_operations;

pub fn render(settings: &mut Settings, ctx: &egui::Context, icons: &Icons) {
    if settings.show_settings {
        let mut save_clicked = false;
        Window::new(RichText::new("Settings").color(settings.get_current_theme().settings_title_color))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let theme = &settings.themes[settings.current_theme_index];
                let mut keys = settings.api_keys.lock().unwrap();

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Fireworks API Key:").strong().color(theme.settings_text_color));
                    ui.text_edit_singleline(&mut keys.fireworks);
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Claude API Key:").strong().color(theme.settings_text_color));
                    ui.text_edit_singleline(&mut keys.claude);
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("GPT API Key:").strong().color(theme.settings_text_color));
                    ui.text_edit_singleline(&mut keys.gpt);
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Theme:").strong().color(theme.settings_text_color));
                    
                    ui.visuals_mut().widgets.inactive.bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.hovered.bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.active.bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.open.bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().selection.bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.noninteractive.bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.inactive.weak_bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.hovered.weak_bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.active.weak_bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.open.weak_bg_fill = theme.theme_dropdown_bg_color;
                    ui.visuals_mut().widgets.noninteractive.weak_bg_fill = theme.theme_dropdown_bg_color;

                    egui::ComboBox::from_label("")
                        .selected_text(RichText::new(&settings.themes[settings.current_theme_index].name).color(theme.dropdown_text_color))
                        .show_ui(ui, |ui| {
                            for (index, theme) in settings.themes.iter().enumerate() {
                                ui.selectable_value(&mut settings.current_theme_index, index, RichText::new(&theme.name).color(theme.dropdown_text_color));
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("UI Scale:").strong().color(theme.settings_text_color));
                    ui.add(egui::Slider::new(&mut settings.temp_ui_scale, 0.5..=4.0).step_by(0.05));
                    //ui.add(DragValue::new(&mut settings.temp_ui_scale).speed(0.05).clamp_range(0.5..=4.0));
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new(RichText::new("Save").color(theme.settings_button_text_color)).fill(theme.settings_button_bg_color)).clicked() {
                        save_clicked = true;
                    }
                    if ui.add(egui::Button::new(RichText::new("Close").color(theme.settings_button_text_color)).fill(theme.settings_button_bg_color)).clicked() {
                        settings.show_settings = false;
                        settings.temp_ui_scale = settings.ui_scale;
                    }
                });
            });

        if save_clicked {
            settings_operations::save_api_keys(settings);
            settings_operations::save_theme(settings);
            settings.ui_scale = settings.temp_ui_scale;
            settings_operations::save_ui_scale(settings);
            ctx.set_pixels_per_point(settings.ui_scale);
        }
    }
}