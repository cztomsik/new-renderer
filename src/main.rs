mod commons;
mod render;

//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

use crate::commons::{Bounds, Color, Pos};
use crate::render::backend::raqote::RaqoteBackend;
use crate::render::Renderer;

fn main() {
    let mut r = Renderer::new(RaqoteBackend::new("out.png".to_string(), 800, 600));
    let c = r.create_container(0);

    r.set_background_color(c, Color { r: 255, g: 0, b: 0, a: 255 });

    r.render_container(
        c,
        &vec![Bounds {
            a: Pos::ZERO,
            b: Pos { x: 100., y: 100. },
        }],
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

