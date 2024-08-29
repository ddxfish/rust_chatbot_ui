use eframe::egui::{self, TextureHandle};

pub struct Icons {
    pub send: TextureHandle,
    pub close: TextureHandle,
    pub new_chat: TextureHandle,
    pub trash: TextureHandle,
}

impl Icons {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            send: load_icon_texture(ctx, "send", include_bytes!("../../assets/paper-plane-regular-white.png")),
            close: load_icon_texture(ctx, "close", include_bytes!("../../assets/close_icon.png")),
            new_chat: load_icon_texture(ctx, "new_chat", include_bytes!("../../assets/app_icon.png")),
            trash: load_icon_texture(ctx, "trash", include_bytes!("../../assets/trash_icon.png")),
        }
    }
}

fn load_icon_texture(ctx: &egui::Context, name: &str, bytes: &[u8]) -> TextureHandle {
    let image = image::load_from_memory(bytes).expect("Failed to load icon").to_rgba8();
    let (width, height) = image.dimensions();
    ctx.load_texture(
        name,
        egui::ColorImage::from_rgba_unmultiplied([width as _, height as _], &image),
        egui::TextureOptions::default()
    )
}