extern crate piston_window;

use piston_window::*;

use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    const BLOCK_SIZE: i32 = 1;

    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why.description())
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {},
        Err(why) => panic!("couldn't read {}: {}", display, why.description())
    }

    let lines = s.split("\n");
    let mut num_rows = -1;
    let mut num_cols = -1;
    let mut max_val = -1.0;
    let mut pixels: Vec<i32> = Vec::new();
    for (i, line) in lines.enumerate() {
        match i {
            0 => {
                if line.trim() != "P3" {
                    panic!("Unsupported magic number: {}", line)
                }
            },
            1 => {
                let dimensions: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                num_cols = dimensions[0];
                num_rows = dimensions[1];
            },
            2 => {
                max_val = line.trim().parse().unwrap();
            }
            _ => {
                let cur_line_of_pixels = line.split_whitespace();
                for x in cur_line_of_pixels {
                    pixels.push(x.parse().unwrap())
                }
            }
        }
    }

    assert!(pixels.len() as i32 == num_cols * num_rows * 3, "Length of pixel array must match num_cols * num_rows * 3. Input is malformed!");

    println!("num_cols: {} num_rows: {}", num_cols, num_rows);
    let window_dimensions = [(num_cols * BLOCK_SIZE) as u32, (num_rows * BLOCK_SIZE) as u32];
    println!("width: {} height: {}", window_dimensions[0], window_dimensions[1]);
    let mut window: PistonWindow = WindowSettings::new("PPM Viewer", window_dimensions)
                                   .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            for row in 0..num_rows {
                for col in 0..num_cols {
                    // TODO: add column_stride constant
                    let red_component = pixels[(num_cols * row * 3 + col * 3) as usize] as f32;
                    let green_component = pixels[(num_cols * row * 3 + col * 3 + 1) as usize] as f32;
                    let blue_component = pixels[(num_cols * row * 3 + col * 3 + 2) as usize] as f32;
                    rectangle([red_component/max_val, green_component/max_val, blue_component/max_val, 1.0],
                        [(col * BLOCK_SIZE) as f64, (row * BLOCK_SIZE) as f64, BLOCK_SIZE as f64, BLOCK_SIZE as f64],
                        context.transform,
                        graphics);
                }
            }
        });
    }
}
