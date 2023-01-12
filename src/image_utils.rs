pub mod image {
	use crate::pixel_utils::pixel::Pixel;
	use std::fs;
	use std::io::Write;

	pub struct Image {
		pub width: usize,
		pub height: usize,
		pub magic_number: String,
		pub scale: u8,
		pub pixels: Vec<Pixel>,
	}

	impl Image {
		//--- UTILITIES --------------------------------------------------------

		/**
		 * Helper function for matrix indexing of the pixel vector.
		 *
		 * Parameters:
		 *   x (usize): x coordinate
		 *   y (usize): y coordinate
		 *
		 * Returns:
		 *   Option<usize>: Optional vector index
		 */
		#[inline]
		fn index(&self, x: usize, y: usize, width: usize) -> Option<usize> {
			if x < self.width && y < self.height {
				return Some(y * width + x);
			}
			None
		}

		/**
		 * Returns optional pixel given matrix coordinates.
		 *
		 * Parameters:
		 *   x (usize): x coordinate
		 *   y (usize): y coordinate
		 *
		 * Returns:
		 *   Option<&Pixel>: Optional pixel
		 */
		pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
			match self.index(x, y, self.width) {
				Some(index) => self.pixels.get(index),
				None => None,
			}
		}

		//--- READING & WRITING ------------------------------------------------

		/**
		 * Returns an image struct parsed from the file.
		 *
		 * Source:
		 *   https://github.com/chris-paterson/PPM
		 *
		 * Parameters:
		 *   file (Path): Path to the file
		 *
		 * Returns:
		 *   Image: Image struct parsed from file
		 */
		pub fn read_file(file: String) -> Image {
			let contents = match fs::read_to_string(file) {
				Ok(str) => str,
				Err(err) => panic!("{:?}", err),
			};
			let lines: Vec<&str> = contents.split('\n').collect();
			let header: Vec<&str> = lines[0..3].to_vec();
			let body: Vec<&str> = lines[3..].to_vec();
			let (magic_number, width, height, scale) =
				match Self::parse_header(&header) {
					Some((m, w, h, s)) => (m, w, h, s),
					None => panic!("Error in parsing the header"),
				};
			let pixels: Vec<Pixel> = match Self::parse_pixels(&body) {
				Some(pixels) => pixels,
				None => panic!("Error in parsing the pixels."),
			};
			Image {
				width,
				height,
				scale,
				magic_number,
				pixels,
			}
		}

		/**
		 * Parse the header of a PPM image file.
		 *
		 * Source:
		 *   https://github.com/chris-paterson/PPM
		 *
		 * Parameters:
		 *   lines (&[&str]): The lines to parse
		 *
		 * Returns:
		 *   Option<(String, usize, usize, u8)>: Parse the magic number and the dimensions of the file.
		 */
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
			Some((magic_number.to_string(), width, height, scale))
		}

		/**
		 * Parse the pixels of the PPM image file.
		 *
		 * Source:
		 *   https://github.com/chris-paterson/PPM
		 *
		 * Parameters:
		 *   lines (&[&str]): The lines to parse
		 *
		 * Returns:
		 *   Option<Vec<Pixel>>: Returns an Optional of a pixel matrix, saved as vector
		 */
		fn parse_pixels(lines: &[&str]) -> Option<Vec<Pixel>> {
			let content: Vec<u8> = lines
				.join(" ")
				.replace('\n', "")
				.replace("  ", " ")
				.trim_end()
				.split(' ')
				.map(|x| x.parse::<u8>().unwrap())
				.collect();
			let mut pixels = Vec::new();
			for i in (0..content.len()).step_by(3) {
				let red = content[i];
				let green = content[i + 1];
				let blue = content[i + 2];
				pixels.push(Pixel { red, green, blue });
			}
			Some(pixels)
		}

		/**
		 * Write an image to a file.
		 *
		 * Source:
		 *   https://github.com/chris-paterson/PPM
		 *
		 * Parameters:
		 *   filename (String): Path to the file
		 */
		pub fn write_file(&self, filename: String) {
			let mut file =
				fs::File::create(filename).expect("Could not write to file");
			writeln!(file, "{}", self.magic_number)
				.expect("Could not write magic number.");
			writeln!(file, "{} {}", self.height, self.width)
				.expect("Could not write height and width.");
			writeln!(file, "{}", self.scale).expect("Could not write scale");

			for y in 0..self.height {
				for x in 0..self.width {
					let pixel = self.get(x, y).unwrap();
					let r = pixel.red;
					let g = pixel.green;
					let b = pixel.blue;
					write!(file, "{} {} {} ", r, g, b)
						.expect("Could not write pixel");
				}
				writeln!(file).expect("Could not write newline!");
			}
		}

		//--- IMAGE STATISTICS -------------------------------------------------

		/**
		 * Returns the brightness of the pixels, defined as the sum of the color channels, divided by three.
		 *
		 * Parameters:
		 *   &self: Image to display the brightness from
		 *
		 * Returns:
		 *   u32: Brightness of the image.
		 */
		fn brightness(&self) -> u32 {
			let num: u32 = (self.width * self.height).try_into().unwrap();
			let mut sum: u32 = 0;

			for pixel in &self.pixels {
				sum += ((pixel.red as u32)
					+ (pixel.green as u32)
					+ (pixel.blue as u32))
					/ 3;
			}

			sum / num
		}

		/**
		 * Print statistics from the image.
		 *
		 * Parameters:
		 *   &self: The image to display statistics from
		 */
		pub fn statistics(&self) {
			println!("Type:       {}", self.magic_number);
			println!("Height:     {}", self.height);
			println!("Width:      {}", self.width);
			println!("Pixels:     {}", self.pixels.len());
			println!("Brightness: {}", self.brightness());
		}

		//--- SEAM CARVING -----------------------------------------------------

		pub fn calculate_energy(
			&self,
			mut energy: Vec<i16>,
			border: usize,
		) -> Vec<i16> {
			// Edge Case: First Element
			energy[0] = 0;

			// Edge Case: First Row
			for j in 1..border {
				let current = self.index(0, j, self.width).unwrap();
				let left = self.index(0, j - 1, self.width).unwrap();

				energy[current] =
					self.pixels[current].color_diff(&self.pixels[left]);
			}

			// Edge Case: Left Border
			for i in 1..self.height {
				let current = self.index(i, 0, self.width).unwrap();
				let above = self.index(i - 1, 0, self.width).unwrap();

				energy[current] =
					self.pixels[current].color_diff(&self.pixels[above]);
			}

			// No Edge Cases
			for i in 1..self.height {
				for j in 1..border {
					let current = self.index(i, j, self.width).unwrap();
					let left = self.index(i, j - 1, self.width).unwrap();
					let above = self.index(i - 1, j, self.width).unwrap();

					energy[current] = self.pixels[current]
						.color_diff(&self.pixels[left])
						+ self.pixels[current].color_diff(&self.pixels[above]);
				}
			}

			// First Row remains unchanged (no upper neighbors)
			for i in 1..self.height {
				let mut current = self.index(i, 0, self.width).unwrap();
				let mut above = self.index(i - 1, 0, self.width).unwrap();
				let mut right = self.index(i - 1, 1, self.width).unwrap();
				let mut left;

				// Edge Case: Left Border
				energy[current] += energy[above].min(energy[right]);

				// No Edge Cases
				for j in 1..border - 1 {
					current = self.index(i, j, self.width).unwrap();
					left = self.index(i - 1, j - 1, self.width).unwrap();
					above = self.index(i - 1, j, self.width).unwrap();
					right = self.index(i - 1, j + 1, self.width).unwrap();

					energy[current] =
						energy[above].min(energy[left].min(energy[right]));
				}

				// Edge Case: Right Border
				current = self.index(i, border - 1, self.width).unwrap();
				above = self.index(i - 1, border - 1, self.width).unwrap();
				left = self.index(i - 1, border - 2, self.width).unwrap();

				energy[current] += energy[left].min(energy[above]);
			}

			energy
		}
	}
}
