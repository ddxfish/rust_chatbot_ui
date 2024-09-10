mod app;
mod chat;
mod ui;
mod message;
mod chatbot;
mod settings;
mod providers;
use eframe::NativeOptions;
use egui::{ViewportBuilder, IconData};

fn load_icon() -> IconData {
    let image = image::load_from_memory(include_bytes!("../assets/app_icon.png"))
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() -> Result<(), eframe::Error> {
    let icon = load_icon();

    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([850.0, 600.0])
            .with_icon(icon),
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Rust Chatbot UI",
        options,
        Box::new(|cc| {
            let settings = settings::Settings::new();
            let theme = settings.get_current_theme().clone();
            cc.egui_ctx.set_visuals(theme.apply_to_visuals());

            Ok(Box::new(app::ChatbotApp::new(cc)))
        })
    )
}