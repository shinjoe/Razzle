use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_file() -> (i32, i32, Vec<f32>) {
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
    let mut pixels: Vec<f32> = Vec::new();
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
                    let color_value: i32 = x.parse().unwrap();
                    pixels.push(color_value as f32/max_val)
                }
            }
        }
    }
    (num_rows, num_cols, pixels)
}