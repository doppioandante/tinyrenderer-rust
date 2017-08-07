mod tga;

use tga::{TGAImage, TGAImage_Format, TGAColor};
use std::ffi::CString;
use std::os::raw::*;

/*
#include "tgaimage.hpp"
const TGAColor white = TGAColor(255, 255, 255, 255);
const TGAColor red   = TGAColor(255, 0,   0,   255);
int main(int argc, char** argv) {
        TGAImage image(100, 100, TGAImage::RGB);
        image.set(52, 41, red);
        image.flip_vertically(); // i want to have the origin at the left bottom corner of the image
        image.write_tga_file("output.tga");`
        return 0;
}
*/

fn main() {
    unsafe {
        let mut image = TGAImage::new1(100, 100, TGAImage_Format::RGB as c_int);
        let color = TGAColor::new1(255, 0, 0, 255);
        image.set2(52, 41, &color);
        image.flip_vertically();
        image.write_tga_file(CString::new("output.tga").unwrap().as_ptr(), true);
        image.destruct();
    }
    println!("Finished writing to output.tga");
}
