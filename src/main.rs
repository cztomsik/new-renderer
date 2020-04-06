mod commons;
mod render;

//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

use crate::commons::{Bounds, Pos};
use crate::render::backend::raqote::RaqoteBackend;
use crate::render::{Border, BorderSide, BorderStyle, Child, Color, Outline, OutlineShadow, OutlineStyle, Renderer};

fn main() {
    let mut r = Renderer::new(RaqoteBackend::new("out.png".to_string(), 800, 600));
    let parent = r.create_container(0);
    let child = r.create_container(1);

    r.insert_child(parent, 0, Child::Container(child));

    r.set_background_color(parent, Color::RED);
    r.set_border(
        parent,
        Some(Border {
            top: None,
            right: None,
            bottom: Some(BorderSide {
                width: 1.,
                style: BorderStyle::Solid,
                color: Color::WHITE,
            }),
            left: None,
        }),
    );

    r.set_background_color(child, Color::GREEN);
    r.set_outline(
        child,
        Some(Outline {
            width: 1.,
            style: OutlineStyle::Solid,
            color: Color::BLUE,
        }),
    );
    r.set_outline_shadows(
        child,
        vec![OutlineShadow {
            offset: Pos::ZERO,
            blur: 0.,
            spread: 5.,
            color: Color {
                r: 127,
                g: 127,
                b: 127,
                a: 127,
            },
        }],
    );

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

    /*
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window("Hello", 800, 600).opengl().build().unwrap();

        let mut event_pump = sdl.event_pump().unwrap();
        loop {
            // TODO: render

            match event_pump.wait_event() {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                _ => {}
            }
        }
    */
}
