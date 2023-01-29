mod energy_utils;
mod image_utils;
mod pixel_utils;
use image_utils::image::Image;
use nalgebra::DMatrix;
use pixel_utils::pixel::Pixel;

extern crate rand;
use clap::{Parser, Subcommand};
use rand::Rng;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[arg(short, long)]
    filename: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    SeamCarve {
        #[arg(short, long)]
        iterations: i32,
    },
    Statistics {},
    Random {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::SeamCarve { iterations }) => {
            println!("Filename:   {}", cli.filename);
            println!("Iterations: {}", *iterations);
        }
        Some(Commands::Statistics {}) => {
            let image = Image::read(cli.filename);
            image.statistics();
        }
        Some(Commands::Random {}) => {
            generate_random_image();
        }
        None => {}
    }
}

fn generate_random_image() {
    let width: usize = 1000;
    let height: usize = 1000;
    let mut pixels: Vec<Pixel> = Vec::with_capacity(width * height);

    for _ in 0..height {
        for _ in 0..width {
            let red: u8 = rand::thread_rng().gen();
            let green: u8 = rand::thread_rng().gen();
            let blue: u8 = rand::thread_rng().gen();
            let pixel: Pixel = Pixel { red, green, blue };
            pixels.push(pixel);
        }
    }

    let image: Image = Image {
        magic_number: "P3".to_string(),
        scale: 255,
        pixels: DMatrix::from_vec(width, height, pixels),
    };
    image.write("test.ppm".to_string());
}
