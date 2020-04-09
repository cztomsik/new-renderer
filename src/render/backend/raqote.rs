use super::{Color, FillStyle, LayerBuilder, RenderBackend};
use crate::commons::{Bounds, Pos};
use raqote::*;

// temporary backend just to test the renderer works properly
// might be a thing in future but now it just writes PNG file

pub struct RaqoteBackend {
    out_file: String,
    dt: DrawTarget,
    layers: Vec<Vec<RenderOp>>,
    textures: Vec<Vec<u8>>,
}

impl RaqoteBackend {
    pub fn new(out_file: String, width: i32, height: i32) -> Self {
        Self {
            out_file,
            dt: DrawTarget::new(width, height),
            layers: Vec::new(),
            textures: Vec::new(),
        }
    }
}

impl RenderBackend for RaqoteBackend {
    type LayerId = LayerId;
    type LayerBuilder = Vec<RenderOp>;
    type TextureId = TextureId;

    fn create_layer(&mut self) -> Self::LayerId {
        self.layers.push(Vec::new());

        self.layers.len() - 1
    }

    fn rebuild_layer_with(&mut self, layer: Self::LayerId, mut f: impl FnMut(&mut Self::LayerBuilder)) {
        f(&mut self.layers[layer]);
    }

    fn render_layer(&mut self, layer: Self::LayerId) {
        //self.dt.clear(Color::BLACK.into());

        render_op(&RenderOp::Layer(layer), &self.layers, &mut self.dt);

        // TODO: render
        //let _data = self.dt.get_data();

        self.dt.write_png(&self.out_file).unwrap();
    }

    fn create_texture(&mut self, width: i32, height: i32, data: Box<[u8]>) -> Self::TextureId {
        assert_eq!(data.len() as i32, width * height * 4, "invalid texture data len");
        self.textures.push(data.into());

        self.textures.len() - 1
    }

    fn update_texture(&mut self, texture: Self::TextureId, mut f: impl FnMut(&mut [u8])) {
        f(&mut self.textures[texture]);
    }
}

impl LayerBuilder<LayerId, TextureId> for Vec<RenderOp> {
    fn push_triangle(&mut self, a: Pos, b: Pos, c: Pos, style: FillStyle<TextureId>) {
        self.push(RenderOp::Triangle(a, b, c, style));
    }

    fn push_layer(&mut self, layer: LayerId) {
        self.push(RenderOp::Layer(layer));
    }
}

fn render_op(op: &RenderOp, layers: &[Vec<RenderOp>], dt: &mut DrawTarget) {
    match op {
        RenderOp::Triangle(a, b, c, style) => {
            let p = {
                let mut pb = PathBuilder::new();
                pb.move_to(a.x, a.y);
                pb.line_to(b.x, b.y);
                pb.line_to(c.x, c.y);
                pb.line_to(a.x, a.y);
                pb.close();

                pb.finish()
            };

            match style {
                FillStyle::SolidColor(color) => dt.fill(&p, &Source::Solid((*color).into()), &DrawOptions::new()),

                _ => println!("TODO: fill {:?}", style),
            }
        }

        RenderOp::Layer(id) => {
            for op in &layers[*id] {
                render_op(op, layers, dt);
            }
        }
    }
}

pub enum RenderOp {
    Triangle(Pos, Pos, Pos, FillStyle<TextureId>),
    Layer(LayerId),
}

type LayerId = usize;
type TextureId = usize;

impl Into<SolidSource> for Color {
    fn into(self) -> SolidSource {
        SolidSource {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
