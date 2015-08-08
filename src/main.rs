extern crate bit_vec;
extern crate image;

use bit_vec::BitVec;
use image::{ GenericImage, ImageBuffer };
use std::fs::File;


struct Pixel {
    width : usize,
    height : usize,
    buffer : Vec<char>,
}

impl Pixel {
    fn new(width : usize, height : usize, dot : char) -> Pixel {
        let size = width*height;
        Pixel {
            width : width,
            height : height,
            buffer : vec![dot; size],
        }
    }

    fn new_empty(width : usize, height : usize) -> Pixel {
        Pixel::new(width, height, ' ')
    }
    
    fn set_dot(&mut self, x : usize, y : usize, dot : char) {
        let offset = y*self.width + x;
        self.buffer[offset] = dot;
    }
    
    fn get_dot(&self, x : usize, y : usize) -> char {
        let offset = y*self.width + x;
        self.buffer[offset]
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

struct Canvas {
    pixmap : Pixmap,
    pixel : Pixel,
    empty_pixel : Pixel,
}

impl Canvas {
    fn new(pixmap : Pixmap, pixel : Pixel) -> Canvas {
        let empty_pixel = Pixel::new_empty(pixel.width, pixel.height);
        Canvas {
            pixmap : pixmap,
            pixel : pixel,
            empty_pixel : empty_pixel,
        }
    }
    
    fn get_charmap_size(&self) -> (usize, usize) {
        let width = self.pixmap.width * self.pixel.width;
        let height = self.pixmap.height * self.pixel.height;
        (width, height)
    }
    
    fn get_charmap_dot(&self, x : usize, y : usize) -> char {
        let pixel_at_x = x / self.pixel.width;
        let pixel_at_y = y / self.pixel.height;
        
        let pixel_x = x % self.pixel.width;
        let pixel_y = y % self.pixel.height;

        let dot = self.pixmap.get(pixel_at_x, pixel_at_y);
        let pixel = if dot { &self.pixel } else { &self.empty_pixel };
        pixel.get_dot(pixel_x, pixel_y)
    }
    
    fn print(&self) {
        let (width, height) = self.get_charmap_size();        
        for y in 0..height {
            for x in 0..width {
                let dot = self.get_charmap_dot(x, y);
                print!("{}", dot);
            }
            println!("");
        }
    }
}

struct Pixmap {
    width : usize,
    height : usize,
    buffer : BitVec,
}

impl Pixmap {
    fn new(width : usize, height : usize) -> Pixmap {
        Pixmap {
            width : width,
            height : height,
            buffer : BitVec::from_elem(width*height, false),
        }
    }
    
    fn set(&mut self, x : usize, y : usize, flag : bool) {
        let offset = y*self.width + x;
        self.buffer.set(offset, flag);
    }
    
    fn get(&self, x : usize, y : usize) -> bool {
        let offset = y*self.width + x;
        self.buffer.get(offset).unwrap()
    }
}

struct RecursiveImage {
    icon : Pixmap,
    buffer : Pixmap,
}

impl RecursiveImage {
    fn draw(icon : Pixmap, depth : usize) -> Pixmap {
        let width = icon.width.pow(depth as u32);
        let height = icon.height.pow(depth as u32);
        let buffer = Pixmap::new(width, height);
        let mut image = RecursiveImage {
            icon : icon,
            buffer : buffer,
        };
        image.draw_pixmap(depth, 0, 0);
        image.buffer
    }
    
    fn draw_pixmap(&mut self, depth : usize, row : usize, col : usize) {
        if 1 == depth {
            for y in 0..(self.icon.height) {
                for x in 0..(self.icon.width) {
                    let draw = self.icon.get(x, y);
                    if draw { 
                        self.buffer.set(col + x, row + y, true);
                    }
                }
            }
        } else {
            let width = self.icon.width.pow(depth as u32);
            let height = self.icon.height.pow(depth as u32);
            let x_step = width / self.icon.width;
            let y_step = height / self.icon.height;
            for y in 0..(self.icon.height) {
                for x in 0..(self.icon.width) {
                    let draw = self.icon.get(x, y);
                    if draw {
                        self.draw_pixmap(depth - 1, row + y * y_step, col + x * x_step);
                    }
                }
            }
        }
    }
}

fn main() {
    let icon = {
        let mut i = Pixmap::new(7, 7);
        i.set(0, 0, false); i.set(1, 0, false); i.set(2, 0, false); i.set(3, 0, false); i.set(4, 0, false); i.set(5, 0, false); i.set(6, 0, false);
        i.set(0, 1, true); i.set(1, 1, true); i.set(2, 1, false); i.set(3, 1, false); i.set(4, 1, true); i.set(5, 1, true); i.set(6, 1, true);
        i.set(0, 2, true); i.set(1, 2, false); i.set(2, 2, true); i.set(3, 2, false); i.set(4, 2, false); i.set(5, 2, false); i.set(6, 2, true);
        i.set(0, 3, true); i.set(1, 3, false); i.set(2, 3, true); i.set(3, 3, false); i.set(4, 3, false); i.set(5, 3, false); i.set(6, 3, true);
        i.set(0, 4, true); i.set(1, 4, false); i.set(2, 4, true); i.set(3, 4, false); i.set(4, 4, true); i.set(5, 4, false); i.set(6, 4, true);
        i.set(0, 5, true); i.set(1, 5, true); i.set(2, 5, false); i.set(3, 5, false); i.set(4, 5, false); i.set(5, 5, true); i.set(6, 5, false);
        i.set(0, 6, false); i.set(1, 6, false); i.set(2, 6, false); i.set(3, 6, false); i.set(4, 6, false); i.set(5, 6, false); i.set(6, 6, false);
        i
    };
        
    let pixmap = RecursiveImage::draw(icon, 5);
    
    let img = ImageBuffer::from_fn(pixmap.width as u32, pixmap.height as u32, |x, y| {
        if pixmap.get(x as usize, y as usize) {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    
//    let ref mut out = File::create("out.png").unwrap();
    let _ = img.save("out.png").ok().expect("Saving image failed");

    /*
    let pixel = {
        let mut pixel = Pixel::new(3, 3, 'O');
        pixel.set_dot(1, 1, ' ');
        pixel
    };
        
    let canvas = Canvas::new(pixmap, pixel);
    canvas.print();
    */
}
