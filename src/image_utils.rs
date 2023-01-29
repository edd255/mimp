pub mod image {
    use crate::pixel_utils::pixel::Pixel;
    use nalgebra::DMatrix;
    use std::fs;
    use std::io::Write;

    pub struct Image {
        pub magic_number: String,
        pub scale: u8,
        pub pixels: DMatrix<Pixel>,
    }

    impl Image {
        //--- READING & WRITING -------------------------------------------------------------------

        ///
        /// Returns an image struct parsed from the file.
        ///
        /// Source:
        ///   https://github.com/chris-paterson/PPM
        ///
        /// Parameters:
        ///   file (Path): Path to the file
        ///
        /// Returns:
        ///   Image: Image struct parsed from file
        ///
        pub fn read(file: String) -> Image {
            let contents = match fs::read_to_string(file) {
                Ok(str) => str,
                Err(err) => panic!("{err:?}"),
            };
            let lines: Vec<&str> = contents.split('\n').collect();
            let header: Vec<&str> = lines[0..3].to_vec();
            let body: Vec<&str> = lines[3..].to_vec();
            let (magic_number, width, height, scale) = match Self::parse_header(&header) {
                Some((m, w, h, s)) => (m, w, h, s),
                None => panic!("Error in parsing the header"),
            };
            let pixels: DMatrix<Pixel> = match Self::parse_pixels(&body, width, height) {
                Some(pixels) => pixels,
                None => panic!("Error in parsing the pixels."),
            };
            Image {
                magic_number,
                scale,
                pixels,
            }
        }

        ///
        /// Parse the header of a PPM image file.
        ///
        /// Source:
        ///   https://github.com/chris-paterson/PPM
        ///
        /// Parameters:
        ///   lines (&[&str]): The lines to parse
        ///
        /// Returns:
        ///   Option<(String, usize, usize, u8)>: Parse the magic number and the dimensions of the
        ///   file.
        ///
        fn parse_header(lines: &[&str]) -> Option<(String, usize, usize, u8)> {
            let magic_number = lines.first().unwrap();
            let dimensions: Vec<&str> = match lines.get(1) {
                Some(dimensions) => dimensions.split(' ').collect(),
                None => return None,
            };
            let width = dimensions[0].parse::<usize>().unwrap();
            let height = dimensions[1].parse::<usize>().unwrap();
            let scale: u8 = match lines.get(2) {
                Some(size) => size.parse::<u8>().unwrap(),
                None => return None,
            };
            Some(((*magic_number).to_string(), width, height, scale))
        }

        ///
        /// Parse the pixels of the PPM image file.
        ///
        /// Source:
        ///   https://github.com/chris-paterson/PPM
        ///
        /// Parameters:
        ///   lines (&[&str]): The lines to parse
        ///
        /// Returns:
        ///   Option<Vec<Pixel>>: Returns an Optional of a pixel matrix, saved as vector
        ///
        fn parse_pixels(lines: &[&str], width: usize, height: usize) -> Option<DMatrix<Pixel>> {
            let content: Vec<u8> = lines
                .join(" ")
                .replace('\n', "")
                .replace("  ", " ")
                .trim_end()
                .split(' ')
                .map(|x| x.parse::<u8>().unwrap())
                .collect();
            let mut pixels: Vec<Pixel> = Vec::new();
            for i in (0..content.len()).step_by(3) {
                let red = content[i];
                let green = content[i + 1];
                let blue = content[i + 2];
                pixels.push(Pixel { red, green, blue });
            }
            Some(DMatrix::from_vec(height, width, pixels))
        }

        ///
        /// Write an image to a file.
        ///
        /// Source:
        ///   https://github.com/chris-paterson/PPM
        ///
        /// Parameters:
        ///   filename (String): Path to the file
        ///
        pub fn write(&self, filename: String) {
            let mut file = fs::File::create(filename).expect("Could not write to file");
            writeln!(file, "{}", self.magic_number).expect("Could not write magic number.");
            writeln!(file, "{} {}", self.pixels.ncols(), self.pixels.nrows())
                .expect("Could not write height and width.");
            writeln!(file, "{}", self.scale).expect("Could not write scale");

            for y in 0..self.pixels.nrows() {
                for x in 0..self.pixels.ncols() {
                    let pixel = &self.pixels[(x, y)];
                    let red = pixel.red;
                    let green = pixel.green;
                    let blue = pixel.blue;
                    write!(file, "{red} {green} {blue}").expect("Could not write pixel");
                }
                writeln!(file).expect("Could not write newline");
            }
        }

        //--- IMAGE STATISTICS --------------------------------------------------------------------

        ///
        /// Returns the brightness of the pixels, defined as the sum of the color channels, divided
        /// by three.
        ///
        /// Parameters:
        ///   &self: Image to display the brightness from
        ///
        /// Returns:
        ///   u32: Brightness of the image.
        ///
        fn brightness(&self) -> u32 {
            let size: u32 = (self.pixels.nrows() * self.pixels.ncols())
                .try_into()
                .unwrap();
            let mut sum: u32 = 0;

            for pixel in &self.pixels {
                sum += (u32::from(pixel.red) + u32::from(pixel.green) + u32::from(pixel.blue)) / 3;
            }

            sum / size
        }

        ///
        /// Print statistics from the image.
        ///
        /// Parameters:
        ///   &self: The image to display statistics from
        ///
        pub fn statistics(&self) {
            println!("Type:       {}", self.magic_number);
            println!("Height:     {}", self.pixels.nrows());
            println!("Width:      {}", self.pixels.ncols());
            println!("Brightness: {}", self.brightness());
        }

        //--- SEAM CARVING ------------------------------------------------------------------------

        pub fn carve_path(&self, border: i32, seam: Vec<i32>) {
            for j in 0..self.pixels.nrows() {
                let col = seam.get(j).unwrap();
                for i in col..border - 1 {
                    self.pixels[(j, i)].red = self.pixels[(j, i + 1)].red;
                    self.pixels[(j, i)].green = self.pixels[(j, i + 1)].green;
                    self.pixels[(j, i)].blue = self.pixels[(j, i + 1)].blue;
                }
            }
        }

        //--- IMAGE MANIPULATION ------------------------------------------------------------------

        ///
        /// Crop an image.
        ///
        /// Parameters:
        ///   filename (String): Path to the file
        ///
        pub fn crop(&self, filename: String, border: usize) {
            assert!(border <= self.pixels.nrows());
            let mut file = fs::File::create(filename).expect("Could not write to file");
            writeln!(file, "{}", self.magic_number).expect("Could not write magic number.");
            writeln!(file, "{} {}", self.pixels.ncols(), self.pixels.nrows())
                .expect("Could not write height and width.");
            writeln!(file, "{}", self.scale).expect("Could not write scale");
            for y in 0..border {
                for x in 0..self.pixels.ncols() {
                    let pixel = &self.pixels[(x, y)];
                    let red = pixel.red;
                    let green = pixel.green;
                    let blue = pixel.blue;
                    write!(file, "{red} {green} {blue}").expect("Could not write pixel");
                }
                writeln!(file).expect("Could not write newline");
            }
        }

        ///
        /// Rotate an image.
        ///
        /// Parameters:
        ///   filename (String): Path to the file
        ///
        pub fn rotate(&self, filename: String) {
            let mut file = fs::File::create(filename).expect("Could not write to file");
            writeln!(file, "{}", self.magic_number).expect("Could not write magic number.");
            writeln!(file, "{} {}", self.pixels.ncols(), self.pixels.nrows())
                .expect("Could not write height and width.");
            writeln!(file, "{}", self.scale).expect("Could not write scale");

            for y in 0..self.pixels.nrows() {
                for x in 0..self.pixels.ncols() {
                    let pixel = &self.pixels[(x, y)];
                    let red = pixel.red;
                    let green = pixel.green;
                    let blue = pixel.blue;
                    write!(file, "{red} {green} {blue}").expect("Could not write pixel");
                }
                writeln!(file).expect("Could not write newline");
            }
        }
    }
}
