extern crate sdl2;

use std::env;
use std::time::{Duration,Instant};
use std::thread::sleep;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

mod fileutil;

struct WindowDimension {
    width: u32,
    height: u32
}

fn main() {
    const FPS: u64 = 60;
    const FRAME_DELAY: Duration = Duration::from_millis(1000 / FPS);

    // size of each pixel when rendered. Defaults to 1, increase if you want to scale up.
    let mut block_size: u32 = 1;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Need to supply name of file as command line argument.");
    }

    if args.len() == 3 {
        block_size = args[2].parse().unwrap();
    }

    let ppm = match fileutil::parse_ppm_file(&args[1]) {
        Ok(ppm) => ppm,
        Err(err) => panic!(err)
    };

    let window_dimensions = WindowDimension { width: ppm.num_cols * block_size, height: ppm.num_rows * block_size };
    println!("{} x {}", window_dimensions.width, window_dimensions.height);

    let sdl_context = sdl2::init().unwrap();
    let video_context = sdl_context.video().unwrap();

    let window = match video_context.window("Razzle", window_dimensions.width, window_dimensions.height)
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

    for row in 0..ppm.num_rows {
        for col in 0..ppm.num_cols {
            let index = (ppm.num_cols * row * fileutil::STRIDE + col * fileutil::STRIDE) as usize;
            canvas.set_draw_color(sdl2::pixels::Color::RGB(ppm.pixels[index] as u8,
                                                           ppm.pixels[index + 1] as u8,
                                                           ppm.pixels[index + 2] as u8));
            canvas.fill_rect(Rect::new((col * block_size) as i32, (row * block_size) as i32, block_size, block_size)).expect("Unable to paint rect");
        }
    }

    canvas.present();

    let mut frame_start: Instant;
    let mut frame_time: Duration;
    let mut events = sdl_context.event_pump().unwrap();
    let mut done = false;
    while !done {
        frame_start = Instant::now();

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

        frame_time = Instant::now().duration_since(frame_start);
        if FRAME_DELAY > frame_time {
            sleep(FRAME_DELAY - frame_time);
        }
    }
}
