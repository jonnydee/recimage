extern crate image;
extern crate recursive_image;

use image::{ ImageBuffer };
use std::fs::File;
use std::path::Path;
use recursive_image::{ Data, Canvas, Pixmap };

fn main() {
    let pixel = Pixmap::from_file("pixel.txt").unwrap();
    let brush = Pixmap::from_file("brush.txt").unwrap();
    let canvas = Canvas::new(pixel, brush);
    let depth = 2;
    make_image("out.png", &canvas, depth);
}

fn make_image(file_name : &str, data : &Data, depth : usize) {
    let (width, height) = data.get_dimension(depth);
    let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        if data.get_point(depth, x as usize, y as usize) {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    
    let ref mut fout = File::create(&Path::new(file_name)).unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG).ok().expect("Saving image failed");
//	let _ = img.save("out.png").ok().expect("Saving image failed");
}