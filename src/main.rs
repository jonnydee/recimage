extern crate bit_vec;
use bit_vec::BitVec;

#[derive(Clone)]
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
    
    fn get_pixel(&self, x : usize, y : usize) -> &Pixel {
        let dot = self.pixmap.get(x, y);
        if dot { 
            &self.pixel
        } else {
            &self.empty_pixel
        }
    }
    
    fn get_charmap_size(&self) -> (usize, usize) {
        let width = self.pixmap.width * self.pixel.width;
        let height = self.pixmap.height * self.pixel.height;
        (width, height)
    }
    
    fn get_charmap_dot(&self, x : usize, y : usize) -> char {
        let pixel_at_x : usize = x / self.pixel.width;
        let pixel_at_y : usize = y / self.pixel.height;
        let pixel = self.get_pixel(pixel_at_x, pixel_at_y);
        
        let pixel_x = x % self.pixel.width;
        let pixel_y = y % self.pixel.height;
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
        match self.buffer.get(offset) {
            Some(flag) => flag,
            None => false
        }
    }
}

struct Board {
    depth : usize,
    pixmap : Pixmap,
    canvas : Pixmap,
}

impl Board {
    fn new(depth : usize, pixmap : Pixmap) -> Board {
        let width = pixmap.width.pow(depth as u32);
        let height = pixmap.height.pow(depth as u32);
        let canvas = Pixmap::new(width, height);
        Board {
            depth : depth,
            pixmap : pixmap,
            canvas : canvas,
        }
    }
    
    fn draw_pixmap(&mut self, depth : usize, row : usize, col : usize) {
        let width = self.pixmap.width.pow(depth as u32);
        let height = self.pixmap.height.pow(depth as u32);
        if 1 == depth {
            for y in 0..(self.pixmap.height) {
                for x in 0..(self.pixmap.width) {
                    let draw = self.pixmap.get(x, y);
                    if draw { 
                        self.canvas.set(col + x, row + y, true);
                    }
                }
            }
        } else {
            let y_step = height / self.pixmap.height;
            let x_step = width / self.pixmap.width;
            for y in 0..(self.pixmap.height) {
                for x in 0..(self.pixmap.width) {
                    let draw = self.pixmap.get(x, y);
                    if draw {
                        self.draw_pixmap(depth - 1, row + y * y_step, col + x * x_step);
                    }
                }
            }
        }
    }
    
    fn draw(&mut self) {
        let depth = self.depth;
        self.draw_pixmap(depth, 0, 0);
    }
}

fn main() {
    let mut pixel = Pixel::new(3, 3, 'O');
    pixel.set_dot(1, 1, ' ');
    let pixel = pixel;
        
    let mut pixmap = Pixmap::new(3, 3);
    pixmap.set(1, 0, true);
    pixmap.set(0, 1, true);
    pixmap.set(1, 1, true);
    pixmap.set(2, 1, true);
    pixmap.set(0, 2, true);
    pixmap.set(2, 2, true);
    let pixmap = pixmap;
    
    let mut board = Board::new(3, pixmap);
    board.draw();
    
    let canvas = Canvas::new(board.canvas, pixel);
    canvas.print();
}
