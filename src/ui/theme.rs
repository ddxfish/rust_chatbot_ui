use egui::{Color32, Rounding, Stroke, Visuals, FontId, FontFamily};

pub struct DarkTheme {
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
    pub message_text_color: Color32,
    pub selected_chat_color: Color32,
    pub unselected_chat_color: Color32,
    pub input_text_color: Color32,
    pub default_font: FontId,
    pub header_font: FontId,
}

impl DarkTheme {
    pub fn new() -> Self {
        Self {
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
        }
    }

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

pub fn custom_light_theme() -> Visuals {
    let mut visuals = Visuals::light();
    visuals.panel_fill = Color32::from_gray(240);
    visuals.window_fill = Color32::from_gray(230);
    visuals.extreme_bg_color = Color32::from_gray(220);
    visuals.widgets.noninteractive.bg_fill = Color32::from_gray(230);
    visuals.widgets.inactive.bg_fill = Color32::from_gray(220);
    visuals.override_text_color = Some(Color32::from_rgb(20, 20, 20));
    visuals
}