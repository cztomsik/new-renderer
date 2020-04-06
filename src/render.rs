// x decoupled from layout, styles, ... (commons & utils are allowed)
// x easy to test (pass vec of bounds)
// x stateful
// x no realloc, no ECS -> create resources, return handles

#![allow(unused_variables, dead_code)]

use crate::commons::{Bounds, Pos};
use std::ops::Index;

// handles
// public but opaque types

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContainerId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextId(usize);

// re-export value types
mod value_types;
pub use self::value_types::*;

// and backend
pub mod backend;
use self::backend::RenderBackend;

// where `RB` is who does the real work
// and `LK` is some key to get layout bounds
pub struct Renderer<RB: RenderBackend, LK> {
    backend: RB,
    ui_state: UiState<LK>,
    keys: Vec<LK>,
}

impl<RB: RenderBackend, LK: Copy> Renderer<RB, LK> {
    pub fn new(backend: RB) -> Self {
        Self {
            backend,
            ui_state: UiState::new(),
            keys: Vec::new(),
        }
    }

    // container
    pub fn create_container(&mut self, layout_key: LK) -> ContainerId {
        self.ui_state.create_container(layout_key)
    }

    pub fn insert_child(&mut self, container: ContainerId, index: usize, child: Child) {
        self.ui_state.children[container.0].insert(index, child);
    }

    pub fn remove_child(&mut self, container: ContainerId, child: Child) {
        self.ui_state.children[container.0].retain(|ch| *ch != child);
    }

    // setters (in order in which they are needed during rendering)

    // TODO: transform

    pub fn set_overflow(&mut self, container: ContainerId, value: Overflow) {
        self.ui_state.overflows[container.0] = value;
    }

    pub fn set_opacity(&mut self, container: ContainerId, value: f32) {
        self.ui_state.opacities[container.0] = value;
    }

    pub fn set_border_radius(&mut self, container: ContainerId, value: Option<BorderRadius>) {
        self.ui_state.border_radii[container.0] = value;
    }

    pub fn set_outline_shadows(&mut self, container: ContainerId, value: Vec<OutlineShadow>) {
        self.ui_state.outline_shadows[container.0] = value;
    }

    pub fn set_outline(&mut self, container: ContainerId, value: Option<Outline>) {
        self.ui_state.outlines[container.0] = value;
    }

    pub fn set_background_color(&mut self, container: ContainerId, value: Color) {
        self.ui_state.background_colors[container.0] = value;
    }

    pub fn set_background_images(&mut self, container: ContainerId, value: Vec<BackgroundImage>) {
        self.ui_state.background_images[container.0] = value;
    }

    pub fn set_inset_shadows(&mut self, container: ContainerId, value: Vec<InsetShadow>) {
        self.ui_state.inset_shadows[container.0] = value;
    }

    // TODO: set_text_shadow

    pub fn set_color(&mut self, container: ContainerId, value: Color) {
        self.ui_state.colors[container.0] = value;
    }

    pub fn set_border(&mut self, container: ContainerId, value: Option<Border>) {
        self.ui_state.borders[container.0] = value;
    }

    // image
    // TODO: put it to some existing/new texture (rect-packing)
    pub fn create_image(width: u32, height: u32) {}
    pub fn set_image_data() {}

    // text
    pub fn create_text() {}
    pub fn set_text_data(/* TODO: texture + glyphs */) {}

    pub fn render_container(&mut self, container: ContainerId, bounds: &impl Index<LK, Output = Bounds>) {
        // TODO: keep ops/pipeline if only transforms were changed
        self.backend.clear();

        let mut ctx = RenderContext {
            backend: &mut self.backend,
            ui_state: &self.ui_state,
            bounds,
            current_bounds: bounds[self.ui_state.layout_keys[container.0]],
        };

        ctx.render_container(container);

        self.backend.render();
    }
}

// internal impl starts here

// data-oriented storage
// TODO: BTreeMap, flags, freelists
struct UiState<LK> {
    layout_keys: Vec<LK>,
    children: Vec<Vec<Child>>,
    opacities: Vec<f32>,
    border_radii: Vec<Option<BorderRadius>>,
    overflows: Vec<Overflow>,
    outline_shadows: Vec<Vec<OutlineShadow>>,
    outlines: Vec<Option<Outline>>,
    background_colors: Vec<Color>,
    background_images: Vec<Vec<BackgroundImage>>,
    inset_shadows: Vec<Vec<InsetShadow>>,
    colors: Vec<Color>,
    borders: Vec<Option<Border>>,
}

impl<LK: Copy> UiState<LK> {
    fn new() -> Self {
        Self {
            layout_keys: Vec::new(),
            children: Vec::new(),
            overflows: Vec::new(),
            opacities: Vec::new(),
            border_radii: Vec::new(),
            outline_shadows: Vec::new(),
            outlines: Vec::new(),
            background_colors: Vec::new(),
            background_images: Vec::new(),
            inset_shadows: Vec::new(),
            colors: Vec::new(),
            borders: Vec::new(),
        }
    }

    fn create_container(&mut self, layout_key: LK) -> ContainerId {
        // TODO: maybe defaults shouldn't be here
        // (accept some ContainerState?)
        self.layout_keys.push(layout_key);
        self.children.push(Vec::new());
        self.overflows.push(Overflow::Visible);
        self.opacities.push(1.);
        self.border_radii.push(None);
        self.outline_shadows.push(Vec::new());
        self.outlines.push(None);
        self.background_colors.push(Color::TRANSPARENT);
        self.background_images.push(Vec::new());
        self.inset_shadows.push(Vec::new());
        self.colors.push(Color::BLACK);
        self.borders.push(None);

        ContainerId(self.background_colors.len() - 1)
    }
}

struct RenderContext<'a, RB: RenderBackend, LK, BS: Index<LK, Output = Bounds>> {
    backend: &'a mut RB,
    ui_state: &'a UiState<LK>,
    bounds: &'a BS,
    current_bounds: Bounds,
}

impl<RB: RenderBackend, LK: Copy, BS: Index<LK, Output = Bounds>> RenderContext<'_, RB, LK, BS> {
    fn render_container(&mut self, container: ContainerId) {
        // TODO: transform
        // TODO: overflow (scroll)
        // TODO: opacity
        // TODO: border_radius (clip downwards, (border/shadow only on this level))

        for s in &self.ui_state.outline_shadows[container.0] {
            self.render_outline_shadow(s);
        }

        if let Some(o) = &self.ui_state.outlines[container.0] {
            self.render_outline(o);
        }

        // TODO: clip if Overflow::Hidden
        // (should be after outline)

        self.render_background_color(self.ui_state.background_colors[container.0]);

        for b in &self.ui_state.background_images[container.0] {
            self.render_background_image(b);
        }

        for s in &self.ui_state.inset_shadows[container.0] {
            self.render_inset_shadow(s);
        }

        println!("{:#?}", &self.ui_state.children[container.0]);

        for ch in &self.ui_state.children[container.0] {
            let prev_bounds = self.current_bounds;

            match ch {
                Child::Container(child_ct) => {
                    self.current_bounds = self.bounds[self.ui_state.layout_keys[child_ct.0]].relative_to(self.current_bounds.a);
                    self.render_container(*child_ct);
                }
                Child::Text(child_text) => self.render_text(*child_text),
            }

            self.current_bounds = prev_bounds;
        }

        if let Some(b) = &self.ui_state.borders[container.0] {
            self.render_border(b);
        }
    }

    fn render_outline_shadow(&mut self, shadow: &OutlineShadow) {
        if shadow.blur != 0. {
            println!("TODO: OutlineShadow blur");
        }

        self.backend.push_rect(self.current_bounds.inflate_uniform(shadow.spread), shadow.color);
    }

    fn render_outline(&mut self, outline: &Outline) {
        let Outline { width, color, .. } = *outline;
        let Bounds { a, b } = self.current_bounds;

        // top
        self.backend.push_rect(
            Bounds {
                a,
                b: Pos {
                    x: b.x + width,
                    y: a.y - width,
                },
            },
            color,
        );

        // right
        self.backend.push_rect(
            Bounds {
                a: Pos { x: b.x + width, y: a.y },
                b: Pos { x: b.x, y: b.y + width },
            },
            color,
        );

        // bottom
        self.backend.push_rect(
            Bounds {
                a: Pos {
                    x: a.x - width,
                    y: b.y + width,
                },
                b,
            },
            color,
        );

        // left
        self.backend.push_rect(
            Bounds {
                a: Pos {
                    x: a.x - width,
                    y: a.y - width,
                },
                b: Pos { x: a.x, y: b.y },
            },
            color,
        );
    }

    fn render_background_color(&mut self, color: Color) {
        if color.a != 0 {
            self.backend.push_rect(self.current_bounds, color);
        }
    }

    fn render_background_image(&mut self, background_image: &BackgroundImage) {
        match background_image {
            BackgroundImage::Image {} => println!("TODO: render image"),
            BackgroundImage::LinearGradient {} => println!("TODO: render linear gradient"),
            BackgroundImage::RadialGradient {} => println!("TODO: render radial gradient"),
        }
    }

    fn render_inset_shadow(&mut self, shadow: &InsetShadow) {
        println!("TODO: render_inset_shadow");
    }

    fn render_text(&mut self, text: TextId) {
        println!("TODO: render_text");
    }

    //fn render_text_shadow(&mut self) {}

    fn render_border(&mut self, border: &Border) {
        // note the border is always inside (it acts like padding in layout)

        // TODO: border_radius

        // TODO: corners (overdraw will be visible with alpha colors)
        // TODO: different edge colors (push_triangle)

        let Bounds { a, b } = self.current_bounds;

        if let Some(BorderSide { width, style, color }) = border.top {
            if style == BorderStyle::Solid {
                self.backend.push_rect(
                    Bounds {
                        a,
                        b: Pos { x: b.x, y: a.y + width },
                    },
                    color,
                )
            }
        }

        if let Some(BorderSide { width, style, color }) = border.right {
            if style == BorderStyle::Solid {
                self.backend.push_rect(
                    Bounds {
                        a: Pos { x: b.x - width, y: a.y },
                        b,
                    },
                    color,
                )
            }
        }

        if let Some(BorderSide { width, style, color }) = border.bottom {
            if style == BorderStyle::Solid {
                self.backend.push_rect(
                    Bounds {
                        a: Pos { x: a.x, y: b.y - width },
                        b,
                    },
                    color,
                )
            }
        }

        if let Some(BorderSide { width, style, color }) = border.left {
            if style == BorderStyle::Solid {
                self.backend.push_rect(
                    Bounds {
                        a,
                        b: Pos { x: a.x + width, y: b.y },
                    },
                    color,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_container() {
        let mut r = create_test_renderer();
        let c = r.create_container(0);

        r.render_container(c, &vec![Bounds::ZERO]);

        assert_eq!(r.backend.log, vec!["clear", "render"]);
    }

    #[test]
    fn outline() {
        let mut r = create_test_renderer();
        let c = r.create_container(0);

        r.set_outline(
            c,
            Some(Outline {
                width: 1.,
                style: OutlineStyle::Solid,
                color: Color::BLUE,
            }),
        );
        r.render_container(
            c,
            &vec![Bounds {
                a: Pos::ZERO,
                b: Pos { x: 100., y: 100. },
            }],
        );

        assert_eq!(
            r.backend.log,
            vec![
                "clear",
                "push_rect Bounds((0.0, 0.0), (101.0, -1.0)) Color { r: 0, g: 0, b: 255, a: 255 }",
                "push_rect Bounds((101.0, 0.0), (100.0, 101.0)) Color { r: 0, g: 0, b: 255, a: 255 }",
                "push_rect Bounds((-1.0, 101.0), (100.0, 100.0)) Color { r: 0, g: 0, b: 255, a: 255 }",
                "push_rect Bounds((-1.0, -1.0), (0.0, 100.0)) Color { r: 0, g: 0, b: 255, a: 255 }",
                "render"
            ]
        );
    }

    #[test]
    fn background_color() {
        let mut r = create_test_renderer();
        let c = r.create_container(0);

        r.set_background_color(c, Color::GREEN);
        r.render_container(
            c,
            &vec![Bounds {
                a: Pos::ZERO,
                b: Pos { x: 100., y: 100. },
            }],
        );

        assert_eq!(
            r.backend.log,
            vec![
                "clear",
                "push_rect Bounds((0.0, 0.0), (100.0, 100.0)) Color { r: 0, g: 255, b: 0, a: 255 }",
                "render"
            ]
        );
    }

    #[test]
    fn children() {
        let mut r = create_test_renderer();
        let parent = r.create_container(0);
        let child = r.create_container(1);

        r.insert_child(parent, 0, Child::Container(child));

        r.set_background_color(child, Color { r: 255, g: 0, b: 0, a: 255 });

        r.render_container(
            parent,
            &vec![
                Bounds {
                    a: Pos::ZERO,
                    b: Pos { x: 100., y: 100. },
                },
                Bounds {
                    a: Pos { x: 50., y: 50. },
                    b: Pos { x: 150., y: 150. },
                },
            ],
        );

        assert_eq!(
            r.backend.log,
            vec![
                "clear",
                "push_rect Bounds((50.0, 50.0), (150.0, 150.0)) Color { r: 255, g: 0, b: 0, a: 255 }",
                "render"
            ]
        );
    }

    #[test]
    fn it_works() {
        let mut r = create_test_renderer();
        let c = r.create_container(0);

        r.set_overflow(c, Overflow::Visible);
        r.set_opacity(c, 0.5);
        r.set_border_radius(
            c,
            Some(BorderRadius {
                top_left: 5.,
                top_right: 5.,
                bottom_right: 5.,
                bottom_left: 5.,
            }),
        );
        r.set_outline_shadows(
            c,
            vec![OutlineShadow {
                offset: Pos::ZERO,
                blur: 5.,
                spread: 5.,
                color: Color::BLACK,
            }],
        );
        r.set_outline(
            c,
            Some(Outline {
                width: 1.,
                style: OutlineStyle::Solid,
                color: Color::BLACK,
            }),
        );
        r.set_background_color(c, Color::BLACK);
        r.set_inset_shadows(
            c,
            vec![InsetShadow {
                offset: Pos::ZERO,
                blur: 5.,
                spread: 5.,
                color: Color::BLACK,
            }],
        );
        r.set_color(c, Color::BLACK);
        r.set_border(
            c,
            Some(Border {
                top: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::RED,
                }),
                right: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::GREEN,
                }),
                bottom: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::BLUE,
                }),
                left: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::YELLOW,
                }),
            }),
        );

        r.render_container(c, &vec![Bounds::ZERO]);

        assert_eq!(
            r.backend.log,
            vec![
                "clear",
                "push_rect Bounds((0.0, 0.0), (1.0, -1.0)) Color { r: 0, g: 0, b: 0, a: 255 }",
                "push_rect Bounds((1.0, 0.0), (0.0, 1.0)) Color { r: 0, g: 0, b: 0, a: 255 }",
                "push_rect Bounds((-1.0, 1.0), (0.0, 0.0)) Color { r: 0, g: 0, b: 0, a: 255 }",
                "push_rect Bounds((-1.0, -1.0), (0.0, 0.0)) Color { r: 0, g: 0, b: 0, a: 255 }",
                "push_rect Bounds((0.0, 0.0), (0.0, 0.0)) Color { r: 0, g: 0, b: 0, a: 255 }",
                "push_rect Bounds((0.0, 0.0), (0.0, 1.0)) Color { r: 255, g: 0, b: 0, a: 255 }",
                "push_rect Bounds((-1.0, 0.0), (0.0, 0.0)) Color { r: 0, g: 255, b: 0, a: 255 }",
                "push_rect Bounds((0.0, -1.0), (0.0, 0.0)) Color { r: 0, g: 0, b: 255, a: 255 }",
                "push_rect Bounds((0.0, 0.0), (1.0, 0.0)) Color { r: 255, g: 255, b: 0, a: 255 }",
                "render"
            ]
        );
    }

    fn create_test_renderer<LK: Copy>() -> Renderer<TestRenderBackend, LK> {
        Renderer::new(TestRenderBackend { log: Vec::new() })
    }

    struct TestRenderBackend {
        log: Vec<String>,
    }

    impl RenderBackend for TestRenderBackend {
        fn clear(&mut self) {
            self.log.push("clear".to_string());
        }

        fn push_rect(&mut self, bounds: Bounds, color: Color) {
            self.log.push(format!("push_rect {:?} {:?}", &bounds, &color));
        }

        fn render(&mut self) {
            self.log.push("render".to_string());
        }
    }
}
