use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;


pub struct Canvas<'a> {
    pub width : usize,
    pub height : usize,
    pub pixmap : &'a Pixmap,
    pub pixel : &'a Pixmap,
    empty_pixel : Pixmap,
}

impl<'a> Canvas<'a> {
    pub fn new(pixmap : &'a Pixmap, pixel : &'a Pixmap) -> Canvas<'a> {
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
    
    pub fn get(&self, x : usize, y : usize) -> bool {
        let pixel_at_x = x / self.pixel.width;
        let pixel_at_y = y / self.pixel.height;
        
        let pixel_x = x % self.pixel.width;
        let pixel_y = y % self.pixel.height;

        let flag = self.pixmap.get(pixel_at_x, pixel_at_y);
        let pixel = if flag { self.pixel } else { &self.empty_pixel };
        pixel.get(pixel_x, pixel_y)
    }
    
    pub fn print(&self) {
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

#[derive(Clone)]
pub struct Pixmap {
    pub width : usize,
    pub height : usize,
    buffer : Vec<Vec<bool>>,
}

impl Pixmap {
    pub fn new(width : usize, height : usize, filled : bool) -> Pixmap {
        Pixmap {
            width : width,
            height : height,
            buffer : vec![vec![filled; width]; height],
        }
    }
    
    pub fn sub_view(&self, x_offset : usize, y_offset : usize, width : usize, height : usize) -> Pixmap {
        let mut view = Pixmap::new(width, height, false);
        for y in 0..height {
            let view_y = y + y_offset;
            for x in 0..width {
                let view_x = x + x_offset;
                if !self.get(view_x, view_y) {
                    continue;
                }
                view.set(x, y, true);
            }
        }
        view
    }

    pub fn copy_to(&self, other : &mut Pixmap, x_offset : usize, y_offset : usize) {
        for y in 0..self.height {
            let other_y = y + y_offset;
            for x in 0..self.width {
                let other_x = x + x_offset;
                if self.get(x, y) {
                    other.set(other_x, other_y, true);
                }
            }
        }
    }
    
    pub fn set(&mut self, x : usize, y : usize, flag : bool) {
        self.buffer[y][x] = flag;
    }
    
    pub fn get(&self, x : usize, y : usize) -> bool {
        self.buffer[y][x]
    }

    pub fn from_file(file_name : &str) -> Result<Pixmap, Error> {
        let f = try!(File::open(file_name));
        let file = BufReader::new(&f);
        
        let mut matrix : Vec<Vec<char>> = vec![];
        let mut width = 0;
        for line in file.lines() {
            let l = line.unwrap();
            let row : Vec<_> = l.chars().collect();
            width = cmp::max(width, row.len());
            matrix.push(row);
        }
        let height = matrix.len();
        
        let mut pixmap = Pixmap::new(width, height, false);
        for (y, row) in matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let flag = *c != ' ';
                if flag {
                    pixmap.set(x, y, true);
                }
            }
        }
        Ok(pixmap)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn draw(brush : &Pixmap, depth : usize) -> Pixmap {
    if 0 == depth {
        brush.clone()
    } else {
        let mut buffer : Pixmap;
        let x_step : usize;
        let y_step : usize;
        {
            let width = brush.width.pow((depth + 1) as u32);
            let height = brush.height.pow((depth + 1) as u32);

            x_step = width / brush.width;
            y_step = height / brush.height;

            buffer = Pixmap::new(width, height, false);
        }

        let mut view : Option<Pixmap> = None;
        for y in 0..brush.height {
            let pos_y = y * y_step;
            for x in 0..brush.width {
                if !brush.get(x, y) {
                    continue;
                }
                let pos_x = x * x_step;
                
                if view.is_none() {
                    view = Some(draw(brush, depth - 1));
                }
                let sub_view = view.as_ref().unwrap();
                sub_view.copy_to(&mut buffer, pos_x, pos_y);
            }
        }
        buffer
    }
}
