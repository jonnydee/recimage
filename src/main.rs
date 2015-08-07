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
        match self.buffer.get(offset) {
            Some(flag) => flag,
            None => false
        }
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
    let icon : Pixmap;
    {
        let mut m_icon = Pixmap::new(3, 3);
        m_icon.set(1, 0, true);
        m_icon.set(0, 1, true);
        m_icon.set(1, 1, true);
        m_icon.set(2, 1, true);
        m_icon.set(0, 2, true);
        m_icon.set(2, 2, true);
        icon = m_icon;
    }
        
    let image = RecursiveImage::draw(icon, 3);
    
    let pixel : Pixel;
    {
        let mut m_pixel = Pixel::new(3, 3, 'O');
        m_pixel.set_dot(1, 1, ' ');
        pixel = m_pixel;
    }
        
    let canvas = Canvas::new(image, pixel);
    canvas.print();
}
