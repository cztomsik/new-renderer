mod commons;
mod render;

//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

use crate::commons::{Bounds, Pos};
use crate::render::backend::raqote::RaqoteBackend;
use crate::render::{BackgroundImage, Border, BorderSide, BorderStyle, Child, Color, Outline, OutlineShadow, OutlineStyle, Renderer};

fn main() {
    let mut r = Renderer::new(RaqoteBackend::new("out.png".to_string(), 800, 600));

    let image = r.create_image(64, 64, gen_checkerboard(64, 64, 16));

    let parent = r.create_container(0);
    let child1 = r.create_container(1);
    let child2 = r.create_container(2);
    let text = r.create_text(3);

    r.set_text_data(text, "Hello".to_string());

    r.insert_child(parent, 0, Child::Container(child1));
    r.insert_child(parent, 1, Child::Container(child2));
    r.insert_child(parent, 2, Child::Text(text));

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

    r.set_background_color(child1, Color::GREEN);
    r.set_outline(
        child1,
        Some(Outline {
            width: 1.,
            style: OutlineStyle::Solid,
            color: Color::BLUE,
        }),
    );
    r.set_outline_shadows(
        child1,
        vec![OutlineShadow {
            offset: Pos::ZERO,
            blur: 0.,
            spread: 5.,
            color: Color { r: 127, g: 127, b: 127, a: 127 },
        }],
    );

    r.set_background_images(child2, vec![BackgroundImage::Image { image }]);

    r.render_container(
        parent,
        &vec![
            Bounds {
                a: Pos { x: 50., y: 50. },
                b: Pos { x: 550., y: 550. },
            },
            Bounds {
                a: Pos { x: 350., y: 50. },
                b: Pos { x: 450., y: 150. },
            },
            Bounds {
                a: Pos { x: 50., y: 50. },
                b: Pos { x: 200., y: 200. },
            },
            Bounds::ZERO,
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

// square_size has to be power of 2
fn gen_checkerboard(width: usize, height: usize, square_size: usize) -> Box<[u8]> {
    let mut data = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let v = if x & square_size != y & square_size { 0xFF } else { 0x00 };
            data.extend(vec![v, v, v, 0xFF]);
        }
    }

    data.into_boxed_slice()
}
