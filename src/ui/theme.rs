use egui::{Visuals, Color32};

pub fn custom_light_theme() -> Visuals {
    let mut visuals = Visuals::light();
    visuals.panel_fill = Color32::from_gray(210);
    visuals.window_fill = Color32::from_gray(200);
    visuals.extreme_bg_color = Color32::from_gray(180);
    visuals.widgets.noninteractive.bg_fill = Color32::from_gray(210);
    visuals.widgets.inactive.bg_fill = Color32::from_gray(180);
    visuals.override_text_color = Some(Color32::from_rgb(60, 60, 60));
    visuals
}