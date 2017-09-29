extern crate tga;

use tga::{Image, Format, Color};

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, mut image: &mut Image, color: &Color) {
    if (x1 - x0).abs() < (y1 - y0).abs() {
        draw_line_2(y0, x0, y1, x1, &mut image, &color, true)
    }
    else {
        draw_line_2(x0, y0, x1, y1, &mut image, &color, false)
    }
}

fn draw_line_2(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: &Color, swap: bool) {
    let range = if x0 < x1 {
        x0 .. x1 + 1
    } else {
        x1 .. x0 + 1
    };

    for x in range {
        let t = (x - x0) as f32/(x1 - x0) as f32;
        let y = y0 + ((y1 - y0) as f32 * t) as i32;

        println!("{}, {}", x, y);
        image.set(
            if swap {y} else {x},
            if swap {x} else {y},
            color
        );
    }
}

fn main() {
    let mut image = Image::with_size(100, 100, Format::RGB);
    let white = Color::new(255, 255, 255, 0);
    //draw_line(50, 50, 80, 50, &mut image, &white);
    //draw_line(50, 50, 65, 65, &mut image, &white);
    //draw_line(50, 50, 50, 80, &mut image, &white);
    draw_line(50, 50, 35, 65, &mut image, &white);
    //draw_line(50, 50, 20, 50, &mut image, &white);
    //draw_line(50, 50, 35, 35, &mut image, &white);
    //draw_line(50, 50, 50, 20, &mut image, &white);
    //draw_line(50, 50, 65, 35, &mut image, &white);
    image.flip_vertically();
    image.write_to_path("output.tga", true);
    println!("Finished writing to output.tga");
}