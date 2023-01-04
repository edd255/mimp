mod image;
mod pixel;
use image::image::Image;

use clap::Parser;

// #[macro_use]
// extern crate scan_fmt;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    iter: u8,

    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    println!("Iterations: {}\nFilename: {}", args.iter, args.filename);
    let image = Image::read_file(args.filename);
    println!(
        "Type: {}\nWidth: {}\nHeight: {}\nPixels: {}",
        image.magic_number, image.width, image.height, image.pixels.len()
    );
}
