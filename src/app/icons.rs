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
            send: load_icon_texture(ctx, "assets/send_icon.png"),
            close: load_icon_texture(ctx, "assets/close_icon.png"),
            new_chat: load_icon_texture(ctx, "assets/app_icon.png"),
            trash: load_icon_texture(ctx, "assets/trash_icon.png"),
        }
    }
}

fn load_icon_texture(ctx: &egui::Context, path: &str) -> TextureHandle {
    let image = image::open(path).expect("Failed to open icon").into_rgba8();
    let (width, height) = image.dimensions();
    ctx.load_texture(
        path,
        egui::ColorImage::from_rgba_unmultiplied([width as _, height as _], &image),
        egui::TextureOptions::default()
    )
}