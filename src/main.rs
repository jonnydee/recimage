extern crate image;
extern crate recursive_image;

use image::{ GenericImage, ImageBuffer };
use std::fs::File;
use std::path::Path;
use recursive_image::{ Canvas, Pixmap };

fn main() {
    let brush = Pixmap::from_file("brush.txt").unwrap();
    let pixel = Pixmap::from_file("pixel.txt").unwrap();
    
    let pixmap = recursive_image::draw(&brush, 2);
    let canvas = Canvas::new(&pixmap, &pixel);
    
    let img = ImageBuffer::from_fn(canvas.width as u32, canvas.height as u32, |x, y| {
        if canvas.get(x as usize, y as usize) {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    
    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG).ok().expect("Saving image failed");
//	let _ = img.save("out.png").ok().expect("Saving image failed");
}
