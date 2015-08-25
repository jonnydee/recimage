// The MIT License (MIT)
// =====================
//
// Copyright © 2015 Johann Duscher (a.k.a. Jonny Dee)
// 
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without
// restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following
// conditions:
// 
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.

use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;


pub trait Data {
    fn get_dimension(&self, depth : usize) -> (usize, usize);
    fn get_point(&self, depth : usize, x : usize, y : usize) -> bool;
}

pub struct Pixmap
{
    width : usize,
    height : usize,
    data : Vec<Vec<bool>>,
}

impl Pixmap {
    
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
        
        let mut data : Vec<_> = vec![vec![false; width]; height];
        for (y, row) in matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let flag = *c != ' ';
                if flag {
                    data[y][x] = true;
                }
            }
        }
        let brush = Pixmap { width : width, height : height, data : data };
        Ok(brush)
    }

}

impl Data for Pixmap {
    
    fn get_dimension(&self, depth : usize) -> (usize, usize) {
        let width = self.width.pow((depth + 1) as u32);
        let height = self.height.pow((depth + 1) as u32);
        (width, height)    
    }
    
    fn get_point(&self, depth : usize, x : usize, y : usize) -> bool {
        if 0 == depth {
            return self.data[y][x];
        }
        
        let (elem_width, elem_height) = self.get_dimension(depth - 1);
        let (elem_x, elem_y) = (x / elem_width, y / elem_height);
        if !self.data[elem_y][elem_x] {
            return false;
        }
        let (sub_x, sub_y) = (x % elem_width, y % elem_height);
        self.get_point(depth - 1, sub_x, sub_y)
    }
    
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Canvas {
    pixel : Pixmap,
    brush : Pixmap,
}

impl Canvas {
    
    pub fn new(pixel : Pixmap, brush : Pixmap) -> Canvas {
        return Canvas {
            pixel : pixel,
            brush : brush,
        }
    }
    
}

impl Data for Canvas {
    
    fn get_dimension(&self, depth : usize) -> (usize, usize) {
        let (width, height) = self.brush.get_dimension(depth);
        (width * self.pixel.width, height * self.pixel.height)
    }
        
    fn get_point(&self, depth : usize, x : usize, y : usize) -> bool {
        if !self.pixel.data[y % self.pixel.height][x % self.pixel.width] {
            return false;
        }
        self.brush.get_point(depth, x / self.pixel.width, y / self.pixel.height)
    }

}
