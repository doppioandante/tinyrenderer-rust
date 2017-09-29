extern crate tga;

use tga::{Image, Format, Color};

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: &Color) {
    for x in x0 .. x1 + 1 {
        let t = (x - x0) as f32/(x1 - x0) as f32;
        let y = y0 + ((y1 - y0) as f32 * t) as i32;
        image.set(
            x,
            y,
            color
        );
    }
}

fn main() {
    let mut image = Image::with_size(100, 100, Format::RGB);
    draw_line(20, 20, 80, 40, &mut image, &Color::new(255, 255, 255, 0));
    image.flip_vertically();
    image.write_to_path("output.tga", true);
    println!("Finished writing to output.tga");
}