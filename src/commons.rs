// common types & things used everywhere

use std::fmt::{self, Debug, Formatter};

/// Application unit (or something similar, unit of measure)
pub type Au = f32;

/// 2D Point
#[derive(Clone, Copy)]
pub struct Pos {
    pub x: Au,
    pub y: Au,
}

impl Pos {
    pub const ZERO: Pos = Self { x: 0., y: 0. };

    pub fn mul(&self, n: Au) -> Pos {
        Pos {
            x: self.x * n,
            y: self.y * n,
        }
    }

    pub fn relative_to(&self, pos: Pos) -> Pos {
        Pos {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

/// Bounding box defined by two points
#[derive(Clone, Copy)]
pub struct Bounds {
    pub a: Pos,
    pub b: Pos,
}

impl Bounds {
    pub const ZERO: Bounds = Self { a: Pos::ZERO, b: Pos::ZERO };

    #[inline]
    pub fn width(&self) -> Au {
        self.b.x - self.a.x
    }

    #[inline]
    pub fn height(&self) -> Au {
        self.b.y - self.a.y
    }

    #[must_use]
    pub fn mul(&self, n: Au) -> Bounds {
        let a = self.a.mul(n);
        let b = self.b.mul(n);

        Bounds { a, b }
    }

    #[must_use]
    pub fn inflate_uniform(&self, n: Au) -> Self {
        Self {
            a: Pos {
                x: self.a.x - n,
                y: self.a.y - n,
            },
            b: Pos {
                x: self.b.x + n,
                y: self.b.y + n,
            },
        }
    }

    pub fn center(&self) -> Pos {
        Pos {
            x: self.a.x + (self.b.x - self.a.x) / 2.,
            y: self.a.y + (self.b.y - self.a.y) / 2.,
        }
    }

    // TODO: rename to `translate`
    pub fn relative_to(&self, pos: Pos) -> Bounds {
        let a = self.a.relative_to(pos);
        let b = self.b.relative_to(pos);

        Bounds { a, b }
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.x > self.a.x && pos.x < self.b.x && pos.y > self.a.y && pos.y < self.b.y
    }
}

impl Debug for Bounds {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Bounds").field(&self.a).field(&self.b).finish()
    }
}
