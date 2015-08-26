// Copyright (c) 2015 Johann Duscher (a.k.a. Jonny Dee). All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification, are
// permitted provided that the following conditions are met:
//
//    1. Redistributions of source code must retain the above copyright notice, this list of
//       conditions and the following disclaimer.
//
//    2. Redistributions in binary form must reproduce the above copyright notice, this list
//       of conditions and the following disclaimer in the documentation and/or other materials
//       provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY JOHANN DUSCHER ''AS IS'' AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL <COPYRIGHT HOLDER> OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
// ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
// ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// The views and conclusions contained in the software and documentation are those of the
// authors and should not be interpreted as representing official policies, either expressed
// or implied, of Johann Duscher.

use recimage::Data;
use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;


pub struct Pixmap
{
    pub width : usize,
    pub height : usize,
    pub data : Vec<Vec<bool>>,
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
