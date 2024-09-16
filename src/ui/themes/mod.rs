mod dark;
mod light;
mod olive_and_tan;
mod dark_yellow;

use egui::{Color32, Visuals};

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub panel_fill: Color32,
    pub window_fill: Color32,
    pub extreme_bg_color: Color32,
    pub override_text_color: Color32,
    pub noninteractive_bg_fill: Color32,
    pub inactive_bg_fill: Color32,
    pub user_message_bg: Color32,
    pub user_message_border: Color32,
    pub bot_message_bg: Color32,
    pub bot_message_border: Color32,
    pub selected_chat_color: Color32,
    pub unselected_chat_color: Color32,
    pub input_text_color: Color32,
    pub settings_text_color: Color32,
    pub settings_button_text_color: Color32,
    pub settings_button_bg_color: Color32,
    pub settings_title_color: Color32,
    pub dropdown_text_color: Color32,
    pub new_chat_button_text_color: Color32,
    pub trash_button_bg_color: Color32,
    pub button_text_color: Color32,
    pub button_bg_color: Color32,
    pub model_provider_dropdown_text_color: Color32,
    pub model_provider_dropdown_bg_color: Color32,
    pub theme_dropdown_bg_color: Color32,
    pub bot_name_text_color: Color32,
    pub user_name_text_color: Color32,
    pub bot_text_color: Color32,
    pub user_text_color: Color32,
    // New fields for code block styling
    pub code_block_bg: Color32,
    pub code_block_border: Color32,
    pub code_block_language_color: Color32,
}

impl Theme {
    pub fn apply_to_visuals(&self) -> Visuals {
        let mut visuals = Visuals::dark();
        visuals.panel_fill = self.panel_fill;
        visuals.window_fill = self.window_fill;
        visuals.extreme_bg_color = self.extreme_bg_color;
        visuals.override_text_color = Some(self.override_text_color);
        visuals.widgets.noninteractive.bg_fill = self.noninteractive_bg_fill;
        visuals.widgets.inactive.bg_fill = self.inactive_bg_fill;
        visuals
    }
}

pub fn get_themes() -> Vec<Theme> {
    vec![
        dark::get_dark_theme(),
        light::get_light_theme(),
        olive_and_tan::get_olive_and_tan_theme(),
        dark_yellow::get_dark_yellow_theme(),
    ]
}