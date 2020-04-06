use super::{Color, RenderBackend};
use crate::commons::Bounds;
use raqote::*;

// temporary backend just to test the renderer works properly
// might be a thing in future but now it just writes PNG file
pub struct RaqoteBackend {
    out: String,
    dt: DrawTarget,
}

impl RaqoteBackend {
    pub fn new(out: String, width: i32, height: i32) -> Self {
        Self {
            out,
            dt: DrawTarget::new(width, height),
        }
    }
}

impl RenderBackend for RaqoteBackend {
    fn clear(&mut self) {
        self.dt.clear(Color::BLACK.into());
    }

    fn push_rect(&mut self, bounds: Bounds, color: Color) {
        let mut pb = PathBuilder::new();
        pb.rect(bounds.a.x, bounds.a.y, bounds.width(), bounds.height());
        self.dt.fill(&pb.finish(), &Source::Solid(color.into()), &DrawOptions::new());
    }

    fn render(&mut self) {
        // TODO: render
        //let _data = self.dt.get_data();

        self.dt.write_png("out.png").unwrap();
    }
}

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
