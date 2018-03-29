extern crate sdl2;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

mod fileutil;

#[allow(unused_must_use)]

fn main() {
    // size of each pixel when rendered. Defaults to 1, increase if you want to zoom in.
    const BLOCK_SIZE: u32 = 1;
    // we're dealing with rgb triplets, so that's our stride.
    const STRIDE: u32 = 3;

    let (num_rows, num_cols, pixels) = fileutil::parse_file();

    assert!(pixels.len() == (num_cols * num_rows * STRIDE) as usize,
            "Length of pixel array must match num_cols * num_rows * {}. Input is malformed!", STRIDE);

    let window_dimensions = [num_cols * BLOCK_SIZE, num_rows * BLOCK_SIZE];
    println!("{} x {}", window_dimensions[0], window_dimensions[1]);

    let sdl_context = sdl2::init().unwrap();
    let video_context = sdl_context.video().unwrap();

    let window = match video_context.window("Razzle", window_dimensions[0], window_dimensions[1])
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err)
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {}", err)
    };

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    canvas.clear();

    for row in 0..num_rows {
        for col in 0..num_cols {
            let index = (num_cols * row * STRIDE + col * STRIDE) as usize;
            canvas.set_draw_color(sdl2::pixels::Color::RGB(pixels[index] as u8,
                                                           pixels[index + 1] as u8,
                                                           pixels[index + 2] as u8));
            canvas.fill_rect(Rect::new(col as i32, row as i32, BLOCK_SIZE, BLOCK_SIZE));
        }
    }

    canvas.present();

    // todo: limit fps
    let mut events = sdl_context.event_pump().unwrap();
    let mut done = false;
    while !done {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => done = true,
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        done = true
                    }
                }
                _ => {}
            }
        }
    }
}
