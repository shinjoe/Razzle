extern crate piston_window;

use piston_window::*;

mod fileutil;

fn main() {
    // size of each pixel when rendered. Defaults to 1, increase if you want to zoom in.
    const BLOCK_SIZE: i32 = 1;
    // we're dealing with rgb triplets, so that's our stride.
    const STRIDE: i32 = 3;

    let (num_rows, num_cols, pixels) = fileutil::parse_file();

    assert!(pixels.len() as i32 == num_cols * num_rows * STRIDE, "Length of pixel array must match num_cols * num_rows * {}. Input is malformed!", STRIDE);

    println!("num_cols: {} num_rows: {}", num_cols, num_rows);
    let window_dimensions = [(num_cols * BLOCK_SIZE) as u32, (num_rows * BLOCK_SIZE) as u32];
    println!("width: {} height: {}", window_dimensions[0], window_dimensions[1]);
    let mut window: PistonWindow = WindowSettings::new("PPM Viewer", window_dimensions)
                                   .exit_on_esc(true).build().unwrap();

    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear(WHITE, graphics);
            for row in 0..num_rows {
                for col in 0..num_cols {
                    let index = (num_cols * row * STRIDE + col * STRIDE) as usize;
                    rectangle([pixels[index], pixels[index + 1], pixels[index + 2], 1.0],
                        [col as f64 , row as f64, BLOCK_SIZE as f64, BLOCK_SIZE as f64],
                        context.transform,
                        graphics);
                }
            }
        });
    }
}
