extern crate bit_vec;
use bit_vec::BitVec;

#[derive(Clone)]
struct Pixel {
    width : usize,
    height : usize,
    buffer : Vec<Vec<char>>,
}

impl Pixel {
    fn new(width : usize, height : usize, dot : char) -> Pixel {
        Pixel {
            width : width,
            height : height,
            buffer : vec![vec![dot; width]; height],
        }
    }

    fn new_empty(width : usize, height : usize) -> Pixel {
        Pixel::new(width, height, ' ')
    }
    
    fn set_dot(&mut self, x : usize, y : usize, dot : char) {
        self.buffer[y][x] = dot;
    }
    
    fn get_dot(&self, x : usize, y : usize) -> char {
        self.buffer[y][x]
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

struct Canvas {
    width : usize,
    height : usize,
    empty_pixel : Pixel,
    pixelbuf : Vec<Pixel>,
}

impl Canvas {
    fn new(width : usize, height : usize, empty_pixel : Pixel) -> Canvas {
        let pixelbuf_size = width * height;
        Canvas {
            width : width,
            height : height,
            empty_pixel : empty_pixel.clone(),
            pixelbuf : vec![empty_pixel; pixelbuf_size],
        }
    }
    
    fn clear(&mut self) {
        *self = Canvas::new(self.width, self.height, self.empty_pixel.clone());
    }

    fn set_pixel(&mut self, x : usize, y : usize, pixel : Pixel) {
        let offset = y * self.width + x;
        self.pixelbuf[offset] = pixel;
    }
    
    fn get_pixel(&self, x : usize, y : usize) -> Pixel {
        let offset = y * self.width + x;
        self.pixelbuf[offset].clone()
    }
    
    fn get_charmap_size(&self) -> (usize, usize) {
        let width = self.width * self.empty_pixel.width;
        let height = self.height * self.empty_pixel.height;
        (width, height)
    }
    
    fn get_charmap_dot(&self, x : usize, y : usize) -> char {
        let pixel_at_x : usize = x / self.empty_pixel.width;
        let pixel_at_y : usize = y / self.empty_pixel.height;
        let pixel = self.get_pixel(pixel_at_x, pixel_at_y);
        
        let pixel_x = x % self.empty_pixel.width;
        let pixel_y = y % self.empty_pixel.height;
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
    canvas : Canvas,
    pixel : Pixel,
}

impl Board {
    fn new(depth : usize, pixmap : Pixmap, pixel : Pixel) -> Board {
        let width = pixmap.width.pow(depth as u32);
        let height = pixmap.height.pow(depth as u32);
        let empty_pixel = Pixel::new_empty(pixel.width, pixel.height);
        let canvas = Canvas::new(width, height, empty_pixel);
        Board {
            depth : depth,
            pixmap : pixmap,
            canvas : canvas,
            pixel : pixel,
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
                        self.canvas.set_pixel(col + x, row + y, self.pixel.clone());
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
    
    let mut board = Board::new(3, pixmap, pixel);
    board.draw();
    board.canvas.print();
}
