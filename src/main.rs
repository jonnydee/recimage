extern crate bit_vec;
extern crate image;

use bit_vec::BitVec;
use image::{ GenericImage, ImageBuffer };


struct Canvas<'a> {
    width : usize,
    height : usize,
    pixmap : &'a Pixmap,
    pixel : &'a Pixmap,
    empty_pixel : Pixmap,
}

impl<'a> Canvas<'a> {
    fn new(pixmap : &'a Pixmap, pixel : &'a Pixmap) -> Canvas<'a> {
        let empty_pixel = Pixmap::new(pixel.width, pixel.height, false);
        let width = pixmap.width * pixel.width;
        let height = pixmap.height * pixel.height;
        Canvas {
            width : width,
            height : height,
            pixmap : &pixmap,
            pixel : &pixel,
            empty_pixel : empty_pixel,
        }
    }
    
    fn get(&self, x : usize, y : usize) -> bool {
        let pixel_at_x = x / self.pixel.width;
        let pixel_at_y = y / self.pixel.height;
        
        let pixel_x = x % self.pixel.width;
        let pixel_y = y % self.pixel.height;

        let dot = self.pixmap.get(pixel_at_x, pixel_at_y);
        let pixel = if dot { self.pixel } else { &self.empty_pixel };
        pixel.get(pixel_x, pixel_y)
    }
    
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let dot = if self.get(x, y) { '*' } else { ' ' };
                print!("{}", dot);
            }
            println!("");
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

struct Pixmap {
    width : usize,
    height : usize,
    buffer : BitVec,
}

impl Pixmap {
    fn new(width : usize, height : usize, filled : bool) -> Pixmap {
        Pixmap {
            width : width,
            height : height,
            buffer : BitVec::from_elem(width*height, filled),
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

//////////////////////////////////////////////////////////////////////////////////////////////////////

struct RecursiveImage<'a> {
    brush : &'a Pixmap,
    buffer : Pixmap,
}

impl<'a> RecursiveImage<'a> {
    fn draw(brush : &'a Pixmap, depth : usize) -> Pixmap {
        let width = brush.width.pow(depth as u32);
        let height = brush.height.pow(depth as u32);
        let buffer = Pixmap::new(width, height, false);
        let mut image = RecursiveImage {
            brush : brush,
            buffer : buffer,
        };
        image.draw_pixmap(depth, 0, 0);
        image.buffer
    }
    
    fn draw_pixmap(&mut self, depth : usize, row : usize, col : usize) {
        if 1 == depth {
            for y in 0..(self.brush.height) {
                for x in 0..(self.brush.width) {
                    let draw = self.brush.get(x, y);
                    if draw { 
                        self.buffer.set(col + x, row + y, true);
                    }
                }
            }
        } else {
            let width = self.brush.width.pow(depth as u32);
            let height = self.brush.height.pow(depth as u32);
            let x_step = width / self.brush.width;
            let y_step = height / self.brush.height;
            for y in 0..(self.brush.height) {
                let new_row = row + y * y_step;
                for x in 0..(self.brush.width) {
                    let new_col = col + x * x_step;
                    let draw = self.brush.get(x, y);
                    if draw {
                        self.draw_pixmap(depth - 1, new_row, new_col);
                    }
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let brush = {
        let mut b = Pixmap::new(8, 7, false);
        b.set(0, 0, false); b.set(1, 0, false); b.set(2, 0, false); b.set(3, 0, false); b.set(4, 0, false); b.set(5, 0, false); b.set(6, 0, false); b.set(7, 0, false);
        b.set(0, 1, true); b.set(1, 1, true); b.set(2, 1, false); b.set(3, 1, false); b.set(4, 1, true); b.set(5, 1, true); b.set(6, 1, true); b.set(7, 1, false);
        b.set(0, 2, true); b.set(1, 2, false); b.set(2, 2, true); b.set(3, 2, false); b.set(4, 2, false); b.set(5, 2, false); b.set(6, 2, true); b.set(7, 2, false);
        b.set(0, 3, true); b.set(1, 3, false); b.set(2, 3, true); b.set(3, 3, false); b.set(4, 3, false); b.set(5, 3, false); b.set(6, 3, true); b.set(7, 3, false);
        b.set(0, 4, true); b.set(1, 4, false); b.set(2, 4, true); b.set(3, 4, false); b.set(4, 4, true); b.set(5, 4, false); b.set(6, 4, true); b.set(7, 4, false);
        b.set(0, 5, true); b.set(1, 5, true); b.set(2, 5, false); b.set(3, 5, false); b.set(4, 5, false); b.set(5, 5, true); b.set(6, 5, false); b.set(7, 5, false);
        b.set(0, 6, false); b.set(1, 6, false); b.set(2, 6, false); b.set(3, 6, false); b.set(4, 6, false); b.set(5, 6, false); b.set(6, 6, false); b.set(7, 6, false);
        b
    };
    
    let pixmap = RecursiveImage::draw(&brush, 3);
    let pixel = Pixmap::new(5, 5, true);
    let canvas = Canvas::new(&pixmap, &pixel);
    
    let img = ImageBuffer::from_fn(canvas.width as u32, canvas.height as u32, |x, y| {
        if canvas.get(x as usize, y as usize) {
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
