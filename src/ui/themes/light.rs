use super::Theme;
use egui::{Color32, FontId, FontFamily};

pub fn get_light_theme() -> Theme {
    Theme {
        name: "Light".to_string(),
        panel_fill: Color32::from_gray(240),
        window_fill: Color32::from_gray(230),
        extreme_bg_color: Color32::from_gray(220),
        override_text_color: Color32::from_rgb(20, 20, 20),
        noninteractive_bg_fill: Color32::from_gray(230),
        inactive_bg_fill: Color32::from_gray(220),
        user_message_bg: Color32::from_rgb(220, 220, 220),
        user_message_border: Color32::from_rgb(0, 100, 200),
        bot_message_bg: Color32::from_rgb(240, 240, 240),
        bot_message_border: Color32::from_rgb(100, 0, 100),
        message_text_color: Color32::from_rgb(20, 20, 20),
        selected_chat_color: Color32::BLUE,
        unselected_chat_color: Color32::BLACK,
        input_text_color: Color32::from_rgb(20, 20, 20),
        default_font: FontId::new(16.0, FontFamily::Proportional),
        header_font: FontId::new(18.0, FontFamily::Proportional),
        settings_text_color: Color32::from_rgb(40, 40, 40),
        settings_button_text_color: Color32::from_rgb(20, 20, 20),
        settings_button_bg_color: Color32::from_rgb(200, 200, 200),
        dropdown_bg_color: Color32::from_rgb(230, 230, 230),
        dropdown_text_color: Color32::from_rgb(20, 20, 20),
        input_bg_color: Color32::from_rgb(250, 250, 250),
        new_chat_button_text_color: Color32::from_rgb(20, 20, 20),
        trash_button_bg_color: Color32::from_rgba_premultiplied(255, 0, 0, 100),
        button_text_color: Color32::from_rgb(20, 20, 20),
        button_bg_color: Color32::from_rgb(200, 200, 200),
        model_provider_dropdown_text_color: Color32::from_rgb(20, 20, 20),
        model_provider_dropdown_bg_color: Color32::from_rgb(230, 230, 230),
        theme_dropdown_bg_color: Color32::from_rgb(230, 230, 230),
        settings_window_title_bg_color: Color32::from_rgb(210, 210, 210),
    }
}