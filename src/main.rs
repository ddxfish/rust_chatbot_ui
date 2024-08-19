mod app;
mod chat;
mod ui;
mod message;
mod chatbot;
mod settings;
mod providers;
use eframe::NativeOptions;
use egui::IconData;
use crate::ui::theme::DarkTheme;

fn load_icon(path: &str) -> IconData {
    let image = image::open(path).expect("Failed to open icon").into_rgba8();
    let (width, height) = image.dimensions();
    IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([850.0, 600.0])
            .with_icon(load_icon("assets/app_icon.png")),
        ..Default::default()
    };
    eframe::run_native(
        "Rust Chatbot UI",
        options,
        Box::new(|cc| {
            let theme = DarkTheme::new();
            cc.egui_ctx.set_visuals(theme.apply_to_visuals());
            Ok(Box::new(app::ChatbotApp::new(cc)))
        })
    )
}