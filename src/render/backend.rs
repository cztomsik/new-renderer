// x don't hide hard-limitations behind leaky abstractions
//   (textures, triangles)
//
// x provide way to render:
//   - transform, clip, opacity
//   x outline shadow (+ radius)
//     x gen image elsewhere
//   x outline
//   x bg color
//   x image
//   x linear/radial gradient
//     x gen image elsewhere
//   x inset shadow (+ radius)
//     x gen image elsewhere
//   x text
//     x layer of msdf quads
//
//   - pseudo text shadow
//     - draw same text many times
//     - or just do it properly with filters (TODO)
//
//   x border
//     x solid
//     x triangle (half of the edge is transparent)
//     x inset/outset
//     x radius corner (solid/dotted/dashed/inset/outset/ridge)
//       - gen image/msdf texture elsewhere
//         (for uniform edges, one should be fine for each style)
//
//   - filter, backdrop-filter (postprocess)
//
// - it should be fast to change text color
//   - not sure yet, maybe shared uniform for color multiplying
//     (and opacity could be just special-case of that)

use super::Color;
use crate::commons::{Bounds, Pos};

// ref impl.
// TODO: maybe it could be run with `cargo test --features=raqote`
pub mod raqote;

// - can fill rects/triangles using specific graphics API like OpenGL
// - simple, can't do high-level optimizations
// - work has to be prepared first, by creating & building a layer
// - layer is an abstract container holding future render operations,
//   useful for something which doesn't change too much
//   so it's possible to cache & compose efficiently
// - layers can reference other layers (can be changed independently)
//
// where `LayerId` and `TK` are impl-specific keys/handles for layers and textures
pub trait RenderBackend {
    type LayerId;
    type LayerBuilder;
    type TextureId;

    // layer is empty unless you initialize it with builder
    // this might sound weird but it allows Renderer to have one root layer
    // allocated even when there's nothing to draw
    fn create_layer(&mut self) -> Self::LayerId;

    // replace layer "contents" (affect referencing layers)
    fn rebuild_layer_with(&mut self, layer: Self::LayerId, f: impl FnMut(&mut Self::LayerBuilder));

    // actually draw something
    fn render_layer(&mut self, layer: Self::LayerId);

    // so there's no copying (vec.into_boxed_slice())
    fn create_texture(&mut self, width: i32, height: i32, data: Box<[u8]>) -> Self::TextureId;

    // needed for atlasing
    fn update_texture(&mut self, texture: Self::TextureId, f: impl FnMut(&mut [u8]));
}

// TODO: maybe it should be rather parametrized by FillStyle
// and maybe it could be associated type too
pub trait LayerBuilder<LK, TK: Copy> {
    // TODO: push/pop transform/clip/opacity

    #[inline]
    fn push_rect(&mut self, bounds: Bounds, style: FillStyle<TK>) {
        let Bounds { a, b } = bounds;

        // LLVM should be able to optimize conditionals inside
        // but it's possible to override this impl entirely if needed
        self.push_triangle(a, b, Pos { x: a.x, y: b.y }, style);
        self.push_triangle(a, b, Pos { x: b.x, y: a.y }, style);
    }

    fn push_triangle(&mut self, a: Pos, b: Pos, c: Pos, style: FillStyle<TK>);

    fn push_layer(&mut self, layer: LK);
}

#[derive(Debug, Clone, Copy)]
pub enum FillStyle<TK: Copy> {
    // not sure if Msdf can draw sharp triangles/rects (bcof. antialiasing)
    // but this is also more convenient (no need to prepare textures, etc.)
    // so it's probably a good idea to keep it anyway
    SolidColor(Color),

    // images, gradients, shadows, ...
    Texture(TK, Bounds),

    // text, radii corners, maybe even paths & preprocessed SVG (later)
    Msdf { texture: TK, uv: Bounds, factor: f32, color: Color },
}
