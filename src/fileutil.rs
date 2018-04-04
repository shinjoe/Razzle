use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct PPM {
    pub num_rows: u32,
    pub num_cols: u32,
    pub pixels: Vec<u32>
}

// we're dealing with rgb triplets, so that's our stride.
pub const STRIDE: u32 = 3;

pub fn parse_ppm_file(location: &str) -> Result<PPM, String> {
    const SUPPORTED_MAGIC_NUMBER: &str = "P3";

    let path = Path::new(location);
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => return Err(format!("Couldn't open {}: {}", display, why.description()))
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {},
        Err(why) => return Err(format!("Couldn't read {}: {}", display, why.description()))
    }

    let lines = s.split("\n");
    let mut rows: u32 = 0;
    let mut cols: u32 = 0;
    let mut pixels: Vec<u32> = Vec::new();
    for (i, line) in lines.enumerate() {
        match i {
            0 => {
                if line.trim() != SUPPORTED_MAGIC_NUMBER {
                    return Err(format!("Unsupported magic number: {}", line));
                }
            },
            1 => {
                let dimensions: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                cols = dimensions[0];
                rows = dimensions[1];
            },
            2 => {
                let max_val: u32 = line.trim().parse().unwrap();
                if max_val != 255 {
                    return Err(format!("Unexpected max pixel color value: {}", max_val));
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

    if pixels.len() != (cols * rows * STRIDE) as usize {
        return Err(format!("Length of pixel array must match num_cols * num_rows * {}. Input is malformed!", STRIDE));
    }

    Ok(PPM { num_rows: rows, num_cols: cols, pixels })
}