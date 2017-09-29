extern crate tga;

use std::fs::File;
use std::io::{BufRead, BufReader};
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
            error -= 2 * (if error < 0 {x0-x1} else {x1-x0});
        }
    }
}

fn main() {
    let mut image = Image::with_size(500, 500, Format::RGB);

    let obj_file = File::open("obj/head.obj").expect("Could not read obj/head.obj");
    let reader = BufReader::new(obj_file);

    let mut vertices = Vec::<[f32; 3]>::new();
    let mut faces = Vec::<[usize; 3]>::new();

    for line in reader.lines() {
        if line.is_ok() {
            let line = line.unwrap();
            let space_idx = line.find(' ').unwrap_or(0);
                
            match &line[..space_idx] {
                "v" => {
                    let res: Vec<f32> = line[(space_idx+1)..]
                                        .split(' ')
                                        .map(|s| s.parse::<f32>().unwrap() )
                                        .collect();
                    assert!(res.len() == 3);
                    vertices.push([res[0], res[1], res[2]]);
                },
                "f" => {
                    let res: Vec<usize> = line[(space_idx+1)..]
                                        .split(' ')
                                        .map(|s| {
                                            let slash_idx = s.find('/').expect("invalid face format");
                                            s[..slash_idx].parse::<usize>().unwrap() 
                                        })
                                        .collect();
                    assert!(res.len() == 3);
                    faces.push([res[0] - 1, res[1] - 1, res[2] - 1]);
                },
                _ => {

                }
            }
        }
    }

    let width = image.width() as f32;
    let height = image.height() as f32;

    for face in faces {
        for i in 0 .. 3 {
            let vert1 = vertices[face[i]];
            let vert2 = vertices[face[(i+1)%3]];

            let x0 = ((vert1[0]+1.)*width*0.5) as i32;
            let y0 = ((vert1[1]+1.)*height*0.5) as i32;
            let x1 = ((vert2[0]+1.)*width*0.5) as i32;
            let y1 = ((vert2[1]+1.)*height*0.5) as i32;

            draw_line(x0, y0, x1, y1, &mut image, &Color::new(255, 255, 255, 255));
        }
    }
    
    image.flip_vertically();
    image.write_to_path("output.tga", true);
    println!("Finished writing to output.tga");
}