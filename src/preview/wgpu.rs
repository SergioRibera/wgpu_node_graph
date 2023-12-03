use eframe::egui_wgpu::{Callback, CallbackTrait};
use eframe::epaint::{PaintCallback, Rect};

use super::shape::Shape;

#[derive(Default)]
pub struct WgpuRenderPreview {
    shape: Shape,
}

pub fn wgpu_callback(rect: Rect, angle: f32) -> PaintCallback {
    PaintCallback {
        rect,
        callback: std::sync::Arc::new(Callback::new_paint_callback(
            rect,
            WgpuRenderPreview::default(),
        )),
    }
}

impl CallbackTrait for WgpuRenderPreview {
    fn paint<'a>(
        &'a self,
        _info: eframe::epaint::PaintCallbackInfo,
        _render_pass: &mut eframe::wgpu::RenderPass<'a>,
        _callback_resources: &'a eframe::egui_wgpu::CallbackResources,
    ) {
    }
}
