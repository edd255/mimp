//! This piece of software contains some basic functionality to manipulate images. It is meant as
//! to study Rust for me.

#[warn(missing_docs)]

/// Seam Carving uses color differences of neighboring pixels as dispensability score. This
/// difference is called energy. This crate contains methods to calculate the energy of an image
/// and to find the optimal path according to this dispensability score.
mod energy_utils;

/// This crate contains the data structure that represents images as pixel matrices and
/// functionalities as cropping, transposing, inverting, rotating, mirroring, and the more advanced
/// functionality of seam carving.
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

    #[arg(short, long)]
    output: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    SeamCarve {
        #[arg(short, long)]
        iterations: usize,
    },
    Statistics {},
    Random {},
    Transpose {},
    Rotate {},
    Invert {},
    Mirror {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::SeamCarve { iterations }) => {
            let mut image = Image::read(&cli.filename);
            image.seam_carve(*iterations, &cli.output);
        }
        Some(Commands::Statistics {}) => {
            let image = Image::read(&cli.filename);
            image.statistics();
        }
        Some(Commands::Random {}) => {
            generate_random_image(&cli.output);
        }
        Some(Commands::Transpose {}) => {
            let image = Image::read(&cli.filename);
            image.transpose(&cli.output.to_string());
        }
        Some(Commands::Rotate {}) => {
            let image = Image::read(&cli.filename);
            image.rotate(&cli.output.to_string());
        }
        Some(Commands::Invert {}) => {
            let mut image = Image::read(&cli.filename);
            image.invert(&cli.output.to_string());
        }
        Some(Commands::Mirror {}) => {
            let image = Image::read(&cli.filename);
            image.mirror(&cli.output.to_string());
        }
        None => {}
    }
}

/// Write a random image to a file called `output`. This method will be replaced by proper testing.
///
/// # Parameters:
///   * `output` - A path to the output file
fn generate_random_image(output: &String) {
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
    image.write(output);
}
