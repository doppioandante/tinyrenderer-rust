extern crate tga;

use tga::{Image, Format, Color};

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: &Color) {
    const STEP: f32 = 0.01; // change to 0.1 to be pathological
    let range_max = (1.0 / STEP) as i32;

    for t in 0 .. range_max + 1 {
        let x_range = (x1 - x0) as f32;
        let y_range = (y1 - y0) as f32;
        image.set(
            x0 + (x_range * STEP * t as f32) as i32,
            y0 + (y_range * STEP * t as f32) as i32,
            color
        );
    }
}

fn main() {
    let mut image = Image::with_size(100, 100, Format::RGB);
    draw_line(20, 20, 60, 40, &mut image, &Color::new(255, 255, 255, 0));
    image.flip_vertically();
    image.write_to_path("output.tga", true);
    println!("Finished writing to output.tga");
}