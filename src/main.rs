mod image_utils;
mod pixel_utils;
use image_utils::image::Image;

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
}
