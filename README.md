# Minimal Image Manipulation Program

At university, I implemented the seam carving algorithm in C as a project for a programming lecture.
To familiarize myself with Rust, I reimplemented the algorithm and added more image processing methods.

## Features

The project supports the following manipulations:

* Rotating
* Cropping
* Inverting
* Transposing
* Mirroring
* Seam carving, vertically and horizontally
* Landfilling

## Usage

Run the CLI with Cargo:

```bash
cargo run -- --help
```

Examples:

```bash
cargo run -- invert --filename input.ppm --output out.ppm
cargo run -- mirror --filename input.ppm --output out.ppm
cargo run -- crop --filename input.ppm --output out.ppm --x1 10 --x2 200 --y1 20 --y2 160
cargo run -- seam-carve --filename input.ppm --output out.ppm --iterations 50 --direction vertical
cargo run -- land-fill --filename input.ppm --output out.ppm --x 10 --y 20 --red 255 --green 0 --blue 0
cargo run -- statistics --filename input.ppm
cargo run -- random --output random.ppm
```

## Input Format

`mimp` supports plain Netpbm PPM files with the `P3` magic number.
The parser accepts comments starting with `#` and arbitrary whitespace between tokens.

Current limitations:

* Binary PPM/P6 files are not supported.
* `maxval` must be in the range `1..=255`.
* Pixel samples must be in the range `0..=maxval`.
* The file must contain exactly `width * height * 3` pixel samples.
