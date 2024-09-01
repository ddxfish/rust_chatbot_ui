use super::Theme;
use egui::{Color32, FontId, FontFamily};

pub fn get_dark_theme() -> Theme {
    Theme {
        name: "Dark".to_string(),
        panel_fill: Color32::from_gray(30),
        window_fill: Color32::from_gray(30),
        extreme_bg_color: Color32::from_gray(25),
        override_text_color: Color32::from_rgb(210, 210, 210),
        noninteractive_bg_fill: Color32::from_gray(30),
        inactive_bg_fill: Color32::from_gray(30),
        user_message_bg: Color32::from_rgb(45, 45, 45),
        user_message_border: Color32::from_rgb(0, 122, 255),
        bot_message_bg: Color32::from_rgb(30, 30, 30),
        bot_message_border: Color32::from_rgb(128, 0, 128),
        message_text_color: Color32::from_rgb(210, 210, 210),
        selected_chat_color: Color32::YELLOW,
        unselected_chat_color: Color32::WHITE,
        input_text_color: Color32::from_rgb(220, 220, 220),
        default_font: FontId::new(16.0, FontFamily::Proportional),
        header_font: FontId::new(18.0, FontFamily::Proportional),
        settings_text_color: Color32::from_rgb(200, 200, 200),
        settings_button_text_color: Color32::from_rgb(240, 240, 240),
        settings_button_bg_color: Color32::from_rgb(60, 60, 60),
        dropdown_bg_color: Color32::from_rgb(40, 40, 40),
        dropdown_text_color: Color32::from_rgb(220, 220, 220),
        input_bg_color: Color32::from_rgb(35, 35, 35),
        new_chat_button_text_color: Color32::from_rgb(240, 240, 240),
        trash_button_bg_color: Color32::from_rgba_premultiplied(255, 0, 0, 100),
        button_text_color: Color32::from_rgb(240, 240, 240),
        button_bg_color: Color32::from_rgb(60, 60, 60),
        model_provider_dropdown_text_color: Color32::from_rgb(220, 220, 220),
        model_provider_dropdown_bg_color: Color32::from_rgb(40, 40, 40),
        theme_dropdown_bg_color: Color32::from_rgb(40, 40, 40),
        settings_window_title_bg_color: Color32::from_rgb(50, 50, 50),
    }
}