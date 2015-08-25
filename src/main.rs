extern crate getopts;
extern crate image;
extern crate recursive_image;

use getopts::Options;
use image::{ ImageBuffer };
use std::env;
use recursive_image::{ Data, Canvas, Pixmap };

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("d", "depth", "set recursion depth", "DEPTH");
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("p", "pixel", "set pixel file name", "NAME");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
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