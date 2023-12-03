use app::Application;
use eframe::{egui, Renderer};

mod app;
mod nodes_context_menu;
mod preview;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        renderer: Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "WGPU NodeGraph",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let gl = cc.gl.as_ref().unwrap();

            Box::new(Application::new(gl))
        }),
    )
}
