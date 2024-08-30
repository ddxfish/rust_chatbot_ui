use egui::{Color32, Visuals, FontId, FontFamily};

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
    pub message_text_color: Color32,
    pub selected_chat_color: Color32,
    pub unselected_chat_color: Color32,
    pub input_text_color: Color32,
    pub default_font: FontId,
    pub header_font: FontId,
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
        },
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
        },
        Theme {
            name: "Olive and Tan".to_string(),
            panel_fill: Color32::from_rgb(67, 84, 58),  // Dark olive green for the sidebar
            window_fill: Color32::from_rgb(240, 234, 214),  // Light tan for the main chat area
            extreme_bg_color: Color32::from_rgb(55, 70, 48),  // Darker olive for extreme backgrounds
            override_text_color: Color32::from_rgb(50, 50, 50),  // Dark text for readability
            noninteractive_bg_fill: Color32::from_rgb(180, 190, 165),  // Lighter olive for non-interactive elements
            inactive_bg_fill: Color32::from_rgb(120, 140, 100),  // Medium olive for inactive elements
            user_message_bg: Color32::from_rgb(255, 243, 224),  // Very light tan for user messages
            user_message_border: Color32::from_rgb(200, 190, 170),  // Darker tan for user message borders
            bot_message_bg: Color32::from_rgb(230, 238, 220),  // Light olive for bot messages
            bot_message_border: Color32::from_rgb(150, 170, 130),  // Medium olive for bot message borders
            message_text_color: Color32::from_rgb(50, 50, 50),  // Dark text for messages
            selected_chat_color: Color32::from_rgb(100, 120, 80),  // Darker olive for selected chat
            unselected_chat_color: Color32::from_rgb(200, 210, 190),  // Light olive for unselected chats
            input_text_color: Color32::from_rgb(50, 50, 50),  // Dark text for input
            default_font: FontId::new(16.0, FontFamily::Proportional),
            header_font: FontId::new(18.0, FontFamily::Proportional),
        },
        Theme {
            name: "Dark Yellow".to_string(),
            panel_fill: Color32::from_rgb(64, 64, 0),
            window_fill: Color32::from_rgb(80, 80, 0),
            extreme_bg_color: Color32::from_rgb(48, 48, 0),
            override_text_color: Color32::from_rgb(255, 255, 200),
            noninteractive_bg_fill: Color32::from_rgb(96, 96, 0),
            inactive_bg_fill: Color32::from_rgb(112, 112, 0),
            user_message_bg: Color32::from_rgb(128, 128, 0),
            user_message_border: Color32::from_rgb(255, 255, 0),
            bot_message_bg: Color32::from_rgb(96, 96, 0),
            bot_message_border: Color32::from_rgb(204, 204, 0),
            message_text_color: Color32::from_rgb(255, 255, 200),
            selected_chat_color: Color32::from_rgb(255, 255, 0),
            unselected_chat_color: Color32::from_rgb(204, 204, 0),
            input_text_color: Color32::from_rgb(255, 255, 200),
            default_font: FontId::new(16.0, FontFamily::Proportional),
            header_font: FontId::new(18.0, FontFamily::Proportional),
        }
    ]
}