use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_ppm_file(ppm_location: &str) -> (u32, u32, Vec<u32>) {
    const SUPPORTED_MAGIC_NUMBER: &str = "P3";

    let path = Path::new(ppm_location);
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
    let mut num_rows: u32 = 0;
    let mut num_cols: u32 = 0;
    let mut pixels: Vec<u32> = Vec::new();
    for (i, line) in lines.enumerate() {
        match i {
            0 => {
                if line.trim() != SUPPORTED_MAGIC_NUMBER {
                    panic!("Unsupported magic number: {}", line)
                }
            },
            1 => {
                let dimensions: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                num_cols = dimensions[0];
                num_rows = dimensions[1];
            },
            2 => {
                let max_val: u32 = line.trim().parse().unwrap();
                if max_val != 255 {
                    panic!("Unexpected max pixel color value: {}", max_val);
                }
            }
            _ => {
                let cur_line_of_pixels = line.split_whitespace();
                for x in cur_line_of_pixels {
                    let color_value: u32 = x.parse().unwrap();
                    pixels.push(color_value)
                }
            }
        }
    }
    (num_rows, num_cols, pixels)
}