use super::{ContainerId, TextId};
use crate::commons::{Color, Pos};

// value types
// part of the public interface but not necessarily how it's stored internally
// similar to respective CSS properties but not the same
// (dimensions are absolute, granularity is different, etc.)

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Child {
    Container(ContainerId),
    Text(TextId),
}

// TODO: should be (f32, f32)
#[derive(Debug)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

#[derive(Debug)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[derive(Debug)]
pub struct OutlineShadow {
    pub offset: Pos,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
}

#[derive(Debug)]
pub struct Outline {
    pub width: f32,
    pub style: BorderStyle,
    pub color: Color,
}

#[derive(Debug)]
pub enum BackgroundImage {
    Image {},
    LinearGradient {},
    RadialGradient {},
}

#[derive(Debug)]
pub struct Border {
    pub top: BorderSide,
    pub right: BorderSide,
    pub bottom: BorderSide,
    pub left: BorderSide,
}

#[derive(Debug)]
pub struct BorderSide {
    pub width: f32,
    pub style: BorderStyle,
    pub color: Color,
}

#[derive(Debug)]
pub enum BorderStyle {
    Solid,
}

#[derive(Debug)]
pub struct InsetShadow {
    pub offset: Pos,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
}
