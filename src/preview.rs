use std::sync::Arc;

use eframe::egui::{self, ComboBox, Ui};
use eframe::epaint::mutex::Mutex;
use eframe::{glow, Renderer};

use self::gl::{gl_callback, RotatingTriangle};
use self::shape::Shape;
use self::wgpu::wgpu_callback;

const DEFAULT_SIZE: f32 = 300.;

mod gl;
mod shape;
mod wgpu;

pub struct PreviewWindow {
    render_backend: Renderer,
    triangle: Arc<Mutex<RotatingTriangle>>,
    angle: f32,
    shape: Shape,
}

impl PreviewWindow {
    pub fn new(ctx: &glow::Context, render_backend: Renderer) -> Self {
        Self {
            render_backend,
            angle: 0.,
            triangle: Arc::new(Mutex::new(RotatingTriangle::new(ctx))),
            shape: Default::default(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let x = ui.available_width() - DEFAULT_SIZE - 50.;
        let y = ui.available_height() - DEFAULT_SIZE - 50.;

        egui::Window::new("Preview")
            .movable(true)
            .resizable(true)
            .title_bar(true)
            .collapsible(true)
            .default_open(true)
            .min_size((DEFAULT_SIZE, DEFAULT_SIZE))
            .default_pos((x, y))
            .show(ctx, |ui| {
                ui.label("Shape:");
                ComboBox::new("select_shape", "")
                    .selected_text(self.shape.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.shape, Shape::Quad, "Quad");
                        ui.selectable_value(&mut self.shape, Shape::Cube, "Cube");
                        ui.selectable_value(&mut self.shape, Shape::Sphere, "Sphere");
                    });
                ui.end_row();
                egui::Frame::canvas(ui.style()).show(ui, |ui| self.render_preview(ui));
            });
    }

    fn render_preview(&mut self, ui: &mut Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        self.angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle;

        match self.render_backend {
            Renderer::Glow => ui
                .painter()
                .add(gl_callback(rect, angle, self.triangle.clone())),
            Renderer::Wgpu => ui.painter().add(wgpu_callback(rect, angle)),
        };
    }
}
