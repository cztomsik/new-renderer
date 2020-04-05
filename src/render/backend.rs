use crate::commons::{Bounds, Color};

/// Low-level renderer, specific to the given graphics api (OpenGL/Vulkan/SW)
/// Knows how to draw primitives, prepared by higher-level `Renderer`
///
/// Backend does the real drawing but it's also very simple and can't do much
/// optimizations and has absolutely no idea about scene or anything else.
/// You don't want to use it directly and so it's useless just by itself.
pub trait RenderBackend {
    fn clear(&mut self);
    fn push_rect(&mut self, bounds: Bounds, color: Color);
    fn render(&mut self);
}

pub mod raqote;
