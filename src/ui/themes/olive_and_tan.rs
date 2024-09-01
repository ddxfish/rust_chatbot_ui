use super::Theme;
use egui::{Color32, FontId, FontFamily};

pub fn get_olive_and_tan_theme() -> Theme {
    Theme {
        name: "Olive and Tan".to_string(),
        panel_fill: Color32::from_rgb(67, 84, 58),
        window_fill: Color32::from_rgb(240, 234, 214),
        extreme_bg_color: Color32::from_rgb(55, 70, 48),
        override_text_color: Color32::from_rgb(50, 50, 50),
        noninteractive_bg_fill: Color32::from_rgb(180, 190, 165),
        inactive_bg_fill: Color32::from_rgb(120, 140, 100),
        user_message_bg: Color32::from_rgb(255, 243, 224),
        user_message_border: Color32::from_rgb(200, 190, 170),
        bot_message_bg: Color32::from_rgb(230, 238, 220),
        bot_message_border: Color32::from_rgb(150, 170, 130),
        message_text_color: Color32::from_rgb(50, 50, 50),
        selected_chat_color: Color32::from_rgb(100, 120, 80),
        unselected_chat_color: Color32::from_rgb(200, 210, 190),
        input_text_color: Color32::from_rgb(50, 50, 50),
        default_font: FontId::new(16.0, FontFamily::Proportional),
        header_font: FontId::new(18.0, FontFamily::Proportional),
        settings_text_color: Color32::from_rgb(50, 50, 50),
        settings_button_text_color: Color32::from_rgb(240, 234, 214),
        settings_button_bg_color: Color32::from_rgb(100, 120, 80),
        dropdown_bg_color: Color32::from_rgb(220, 230, 200),
        dropdown_text_color: Color32::from_rgb(50, 50, 50),
        input_bg_color: Color32::from_rgb(250, 245, 230),
        new_chat_button_text_color: Color32::from_rgb(240, 234, 214),
        trash_button_bg_color: Color32::from_rgba_premultiplied(200, 100, 100, 100),
        button_text_color: Color32::from_rgb(240, 234, 214),
        button_bg_color: Color32::from_rgb(100, 120, 80),
        model_provider_dropdown_text_color: Color32::from_rgb(50, 50, 50),
        model_provider_dropdown_bg_color: Color32::from_rgb(220, 230, 200),
        theme_dropdown_bg_color: Color32::from_rgb(220, 230, 200),
        settings_window_title_bg_color: Color32::from_rgb(180, 190, 165),
    }
}