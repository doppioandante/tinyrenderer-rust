extern crate tga;

use tga::{Image, Format, Color};

fn draw_line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut image: &mut Image, color: &Color) {
    // always draw along the largest direction
    let swap = (x1 - x0).abs() < (y1 - y0).abs();
    if swap {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    // always have x0 < x1 in drawing order
    if x0 > x1 {
        std::mem::swap(&mut y0, &mut y1);
        std::mem::swap(&mut x0, &mut x1);
    };

    let derror = y1 - y0;
    let ystep = if y0 < y1 { 1 } else { -1 };
    let mut error: i32 = 0;

    let mut y: i32 = y0;
    for x in x0 .. x1 + 1 {
        if swap {
            image.set(y, x, color);    
        }
        else {
            image.set(x, y, color);
        }
        error += 2 * derror;

        if error.abs() > x1-x0 { // x1-x0 is always positive
            y += ystep;
            error -= 2*(if error < 0 {x0-x1} else {x1-x0});
        }
    }
}

fn main() {
    let mut image = Image::with_size(100, 100, Format::RGB);
    let white = Color::new(255, 255, 255, 0);
    draw_line(50, 50, 80, 50, &mut image, &white);
    draw_line(50, 50, 65, 65, &mut image, &white);
    draw_line(50, 50, 50, 80, &mut image, &white);
    draw_line(50, 50, 35, 65, &mut image, &white);
    draw_line(50, 50, 20, 50, &mut image, &white);
    draw_line(50, 50, 35, 35, &mut image, &white);
    draw_line(50, 50, 50, 20, &mut image, &white);
    draw_line(50, 50, 65, 35, &mut image, &white);
    image.flip_vertically();
    image.write_to_path("output.tga", true);
    println!("Finished writing to output.tga");
}