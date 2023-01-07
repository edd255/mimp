mod image_utils;
mod pixel_utils;
use image_utils::image::Image;
use pixel_utils::pixel::Pixel;

use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::SeamCarve { iterations }) => {
            println!("Filename:   {}", cli.filename);
            println!("Iterations: {}", *iterations);
        }
        Some(Commands::Statistics {}) => {
            let image = Image::read_file(cli.filename);
            image.statistics();
        }
        None => {}
    }

    // Trying out other procedures, before testing them properly later
    let mut a: Pixel = Pixel {
        red: 122,
        green: 100,
        blue: 20,
    };
    let b: Pixel = Pixel {
        red: 80,
        green: 30,
        blue: 50,
    };
    println!();
    println!("Pixel a");
    println!("-----------");
    println!("Red:   {}", a.red);
    println!("Green: {}", a.green);
    println!("Blue:  {}\n", a.blue);
    println!("Pixel b");
    println!("-----------");
    println!("Red:   {}", b.red);
    println!("Green: {}", b.green);
    println!("Blue:  {}\n", b.blue);
    println!("Color diff: {}", a.color_diff(b));
    a.invert();
    println!("Red:        {}", a.red);
    println!("Green:      {}", a.green);
    println!("Blue:       {}", a.blue);
}
