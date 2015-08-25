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

extern crate getopts;
extern crate image;
extern crate recimage;

use getopts::Options;
use image::{ ImageBuffer };
use std::env;
use recimage::{ Data, Canvas, Pixmap };

const AUTHOR: &'static str = "Jonny Dee <jonny.dee@gmx.net>";
const COPYRIGHT_YEARS: &'static str = "2015";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("d", "depth", "set recursion depth", "DEPTH");
    opts.optopt("o", "output", "set output image file name", "FILE");
    opts.optopt("p", "pixel", "set input pixel file name", "FILE");
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