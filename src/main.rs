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

extern crate getopts;
extern crate image;
extern crate recursive_image;

use getopts::Options;
use image::{ ImageBuffer };
use std::env;
use recursive_image::{ Data, Canvas, Pixmap };

const AUTHOR: &'static str = "Jonny Dee <jonny.dee@gmx.net>";
const COPYRIGHT_YEARS: &'static str = "2015";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("d", "depth", "set recursion depth", "DEPTH");
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("p", "pixel", "set pixel file name", "NAME");
    opts.optflag("v", "", "print version number");
    opts.optflag("", "version", "print full version information");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    if matches.opt_present("v") {
        print_version(&program, false);
        return;
    } else if matches.opt_present("version") {
        print_version(&program, true);
        return;
    }
        
    let input_file_name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    
    let depth = match matches.opt_str("d") {
        Some(depth) => depth.parse::<usize>().unwrap(),
        None => 0
    };
    
    let output_file_name = match matches.opt_str("o") {
        Some(file) => file,
        None => input_file_name.clone() + ".png"
    };
    
    let pixel_file_name = match matches.opt_str("p") {
        Some(file) => file,
        None => String::new()
    };
        
    if !pixel_file_name.is_empty() {
        let pixel = Pixmap::from_file(&pixel_file_name).unwrap();
        let brush = Pixmap::from_file(&input_file_name).unwrap();
        let canvas = Canvas::new(pixel, brush);
        make_image(&output_file_name, &canvas, depth);
    } else {
        let brush = Pixmap::from_file(&input_file_name).unwrap();
        make_image(&output_file_name, &brush, depth);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn print_version(program: &str, full : bool) {
    if full {
        println!("{} v{} - Copyright (c) {} {}", program, VERSION, COPYRIGHT_YEARS, AUTHOR);
    } else {
        println!("{}", VERSION);
    }
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
    
    img.save(file_name).ok().expect("Saving image failed");
}