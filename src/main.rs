//! This piece of software contains some basic functionality to manipulate images. It is meant as
//! to study Rust for me.

use clap::{Parser, Subcommand, ValueEnum};
use mimp::pixel_utils::pixel::Pixel;
use mimp::{Image, Rect, SeamDirection};
use nalgebra::DMatrix;
use rand::Rng;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SeamCarve {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long)]
        output: String,

        #[arg(short, long)]
        iterations: usize,

        #[arg(short, long, value_enum)]
        direction: DirectionArg,
    },
    Statistics {
        #[arg(short, long)]
        filename: String,
    },
    Random {
        #[arg(short, long)]
        output: String,
    },
    Transpose {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long)]
        output: String,
    },
    Rotate {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long)]
        output: String,
    },
    Invert {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long)]
        output: String,
    },
    Mirror {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long)]
        output: String,
    },
    Crop {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long)]
        output: String,

        #[arg(long)]
        x1: usize,

        #[arg(long)]
        x2: usize,

        #[arg(long)]
        y1: usize,

        #[arg(long)]
        y2: usize,
    },
    LandFill {
        #[arg(short, long)]
        filename: String,

        #[arg(short, long)]
        output: String,

        #[arg(long)]
        x: usize,

        #[arg(long)]
        y: usize,

        #[arg(short, long)]
        red: u8,

        #[arg(short, long)]
        green: u8,

        #[arg(short, long)]
        blue: u8,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum DirectionArg {
    Vertical,
    Horizontal,
}

impl From<DirectionArg> for SeamDirection {
    fn from(direction: DirectionArg) -> Self {
        match direction {
            DirectionArg::Vertical => Self::Vertical,
            DirectionArg::Horizontal => Self::Horizontal,
        }
    }
}

fn main() -> Result<(), mimp::Error> {
    let cli = Cli::parse();
    match cli.command {
        Commands::SeamCarve {
            filename,
            output,
            iterations,
            direction,
        } => {
            let image = Image::read(&filename)?;
            let carved = image.seam_carve(iterations, direction.into())?;
            carved.write(&output)
        }
        Commands::Statistics { filename } => {
            let image = Image::read(&filename)?;
            image.statistics();
            Ok(())
        }
        Commands::Random { output } => generate_random_image(&output),
        Commands::Transpose { filename, output } => {
            let image = Image::read(&filename)?;
            image.transpose().write(&output)
        }
        Commands::Rotate { filename, output } => {
            let image = Image::read(&filename)?;
            image.rotate().write(&output)
        }
        Commands::Invert { filename, output } => {
            let image = Image::read(&filename)?;
            image.invert().write(&output)
        }
        Commands::Mirror { filename, output } => {
            let image = Image::read(&filename)?;
            image.mirror().write(&output)
        }
        Commands::Crop {
            filename,
            output,
            x1,
            x2,
            y1,
            y2,
        } => {
            let image = Image::read(&filename)?;
            image.crop(Rect { x1, x2, y1, y2 })?.write(&output)
        }
        Commands::LandFill {
            filename,
            output,
            x,
            y,
            red,
            green,
            blue,
        } => {
            let image = Image::read(&filename)?;
            image.landfill((x, y), (red, green, blue))?.write(&output)
        }
    }
}

/// Write a random image to a file called `output`.
///
/// # Parameters:
///   * `output` - A path to the output file
fn generate_random_image(output: &str) -> Result<(), mimp::Error> {
    let width: usize = 1000;
    let height: usize = 1000;
    let mut rng = rand::thread_rng();
    let image = Image {
        magic_number: "P3".to_string(),
        scale: 255,
        pixels: DMatrix::from_fn(height, width, |_, _| Pixel {
            red: rng.gen(),
            green: rng.gen(),
            blue: rng.gen(),
        }),
    };
    image.write(output)
}
