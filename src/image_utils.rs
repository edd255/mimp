//! This crate contains the data structure that represents images as pixel matrices and
//! functionalities as cropping, rotating, inverting and seam carving.

pub mod image {
    use crate::energy_utils::energy;
    use crate::pixel_utils::pixel::Pixel;
    use nalgebra::DMatrix;
    use std::error::Error;
    use std::fmt;
    use std::fs;
    use std::io::Write;

    /// Images in the PPM format have a `magic_number`, e.g. P3 for Portable Pixmaps (ASCII), and a
    /// `scale` is the maximum value for each color. Images are represented as pixel matrices, here
    /// in `pixels`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Image {
        pub magic_number: String,
        pub scale: u8,
        pub pixels: DMatrix<Pixel>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Rect {
        pub x1: usize,
        pub x2: usize,
        pub y1: usize,
        pub y2: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SeamDirection {
        Vertical,
        Horizontal,
    }

    pub type ImageResult<T> = Result<T, ImageError>;

    #[derive(Debug)]
    pub enum ImageError {
        Io(std::io::Error),
        MissingToken(&'static str),
        InvalidMagicNumber(String),
        InvalidNumber {
            field: &'static str,
            value: String,
        },
        InvalidDimensions {
            width: usize,
            height: usize,
        },
        DimensionOverflow,
        UnsupportedMaxval(u16),
        InvalidSampleCount {
            expected: usize,
            actual: usize,
        },
        PixelOutOfRange {
            value: u16,
            maxval: u16,
        },
        InvalidCrop {
            x1: usize,
            x2: usize,
            y1: usize,
            y2: usize,
            width: usize,
            height: usize,
        },
        CoordinateOutOfBounds {
            x: usize,
            y: usize,
            width: usize,
            height: usize,
        },
        InvalidSeamIterations {
            iterations: usize,
            max: usize,
            direction: SeamDirection,
        },
    }

    impl fmt::Display for ImageError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Io(err) => write!(f, "I/O error: {err}"),
                Self::MissingToken(field) => write!(f, "missing PPM {field}"),
                Self::InvalidMagicNumber(magic_number) => {
                    write!(
                        f,
                        "unsupported PPM magic number {magic_number:?}; expected \"P3\""
                    )
                }
                Self::InvalidNumber { field, value } => {
                    write!(f, "invalid PPM {field}: {value:?}")
                }
                Self::InvalidDimensions { width, height } => {
                    write!(f, "invalid image dimensions {width}x{height}")
                }
                Self::DimensionOverflow => write!(f, "image dimensions overflow usize"),
                Self::UnsupportedMaxval(maxval) => {
                    write!(f, "unsupported PPM maxval {maxval}; expected 1..=255")
                }
                Self::InvalidSampleCount { expected, actual } => {
                    write!(
                        f,
                        "invalid PPM sample count: expected {expected}, got {actual}"
                    )
                }
                Self::PixelOutOfRange { value, maxval } => {
                    write!(f, "pixel sample {value} exceeds maxval {maxval}")
                }
                Self::InvalidCrop {
                    x1,
                    x2,
                    y1,
                    y2,
                    width,
                    height,
                } => write!(
                    f,
                    "invalid crop ({x1}, {y1})..({x2}, {y2}) for image {width}x{height}",
                ),
                Self::CoordinateOutOfBounds {
                    x,
                    y,
                    width,
                    height,
                } => write!(f, "coordinate ({x}, {y}) is outside image {width}x{height}"),
                Self::InvalidSeamIterations {
                    iterations,
                    max,
                    direction,
                } => write!(
                    f,
                    "cannot remove {iterations} {direction:?} seams; maximum is {max}",
                ),
            }
        }
    }

    impl Error for ImageError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                Self::Io(err) => Some(err),
                _ => None,
            }
        }
    }

    impl From<std::io::Error> for ImageError {
        fn from(err: std::io::Error) -> Self {
            Self::Io(err)
        }
    }

    impl Image {
        //=== READING & WRITING ===================================================================

        /// Returns an image struct, parsed from a file.
        ///
        /// # Parameters:
        ///  `file` - The location of the file, as a String
        ///
        /// # Returns:
        ///  `Image` - Representation of the image file with the struct Image
        pub fn read(file: &str) -> ImageResult<Image> {
            let contents = fs::read_to_string(file)?;
            Self::from_ppm_str(&contents)
        }

        /// Parses a plain PPM/P3 image from a string.
        pub fn from_ppm_str(contents: &str) -> ImageResult<Image> {
            let tokens = Self::ppm_tokens(contents);
            let mut tokens = tokens.into_iter();

            let magic_number = Self::next_token(&mut tokens, "magic number")?;
            if magic_number != "P3" {
                return Err(ImageError::InvalidMagicNumber(magic_number));
            }

            let width = Self::parse_usize(Self::next_token(&mut tokens, "width")?, "width")?;
            let height = Self::parse_usize(Self::next_token(&mut tokens, "height")?, "height")?;
            if width == 0 || height == 0 {
                return Err(ImageError::InvalidDimensions { width, height });
            }

            let maxval = Self::parse_u16(Self::next_token(&mut tokens, "maxval")?, "maxval")?;
            if maxval == 0 || maxval > u16::from(u8::MAX) {
                return Err(ImageError::UnsupportedMaxval(maxval));
            }

            let pixel_count = width
                .checked_mul(height)
                .ok_or(ImageError::DimensionOverflow)?;
            let expected_samples = pixel_count
                .checked_mul(3)
                .ok_or(ImageError::DimensionOverflow)?;
            let sample_tokens: Vec<String> = tokens.collect();
            if sample_tokens.len() != expected_samples {
                return Err(ImageError::InvalidSampleCount {
                    expected: expected_samples,
                    actual: sample_tokens.len(),
                });
            }

            let mut pixels = Vec::with_capacity(pixel_count);
            for chunk in sample_tokens.chunks(3) {
                let red = Self::parse_sample(&chunk[0], maxval)?;
                let green = Self::parse_sample(&chunk[1], maxval)?;
                let blue = Self::parse_sample(&chunk[2], maxval)?;
                pixels.push(Pixel { red, green, blue });
            }

            Ok(Image {
                magic_number,
                scale: maxval as u8,
                pixels: DMatrix::from_fn(height, width, |row, col| pixels[row * width + col]),
            })
        }

        /// Write an image to a file.
        ///
        /// # Parameters:
        ///  `filename` - path to the file
        pub fn write(&self, filename: &str) -> ImageResult<()> {
            let file = fs::File::create(filename)?;
            self.write_ppm(file)
        }

        /// Writes this image as a plain PPM/P3 stream.
        pub fn write_ppm<W: Write>(&self, mut writer: W) -> ImageResult<()> {
            writeln!(writer, "{}", self.magic_number)?;
            writeln!(writer, "{} {}", self.pixels.ncols(), self.pixels.nrows())?;
            writeln!(writer, "{}", self.scale)?;
            for y in 0..self.pixels.nrows() {
                for x in 0..self.pixels.ncols() {
                    let pixel = self.pixels[(y, x)];
                    write!(writer, "{} {} {} ", pixel.red, pixel.green, pixel.blue)?;
                }
                writeln!(writer)?;
            }
            Ok(())
        }

        fn ppm_tokens(contents: &str) -> Vec<String> {
            contents
                .lines()
                .flat_map(|line| {
                    let data = line.split_once('#').map_or(line, |(data, _)| data);
                    data.split_whitespace().map(str::to_string)
                })
                .collect()
        }

        fn next_token<I>(tokens: &mut I, field: &'static str) -> ImageResult<String>
        where
            I: Iterator<Item = String>,
        {
            tokens.next().ok_or(ImageError::MissingToken(field))
        }

        fn parse_usize(value: String, field: &'static str) -> ImageResult<usize> {
            value
                .parse::<usize>()
                .map_err(|_| ImageError::InvalidNumber { field, value })
        }

        fn parse_u16(value: String, field: &'static str) -> ImageResult<u16> {
            value
                .parse::<u16>()
                .map_err(|_| ImageError::InvalidNumber { field, value })
        }

        fn parse_sample(value: &str, maxval: u16) -> ImageResult<u8> {
            let sample =
                Self::parse_u16(value.to_string(), "pixel sample").map_err(|err| match err {
                    ImageError::InvalidNumber { value, .. } => ImageError::InvalidNumber {
                        field: "pixel sample",
                        value,
                    },
                    err => err,
                })?;
            if sample > maxval {
                return Err(ImageError::PixelOutOfRange {
                    value: sample,
                    maxval,
                });
            }
            Ok(sample as u8)
        }

        //=== IMAGE STATISTICS ====================================================================

        /// Returns the brightness of the pixels, defined as the sum of the color channels, divided
        /// by three.
        ///
        /// # Returns:
        ///  `u32`-  Brightness of the image
        fn brightness(&self) -> u32 {
            let size = self.pixels.nrows() * self.pixels.ncols();
            if size == 0 {
                return 0;
            }
            let mut sum: u32 = 0;
            for pixel in &self.pixels {
                sum += (u32::from(pixel.red) + u32::from(pixel.green) + u32::from(pixel.blue)) / 3;
            }
            sum / size as u32
        }

        /// Print statistics from the image.
        pub fn statistics(&self) {
            println!("Type:       {}", self.magic_number);
            println!("Height:     {}", self.pixels.nrows());
            println!("Width:      {}", self.pixels.ncols());
            println!("Brightness: {}", self.brightness());
        }

        //=== SEAM CARVING ========================================================================

        /// Seam carves an image.
        pub fn seam_carve(
            &self,
            iterations: usize,
            direction: SeamDirection,
        ) -> ImageResult<Image> {
            match direction {
                SeamDirection::Vertical => self.seam_carve_vertical(iterations),
                SeamDirection::Horizontal => self.seam_carve_horizontal(iterations),
            }
        }

        fn seam_carve_vertical(&self, iterations: usize) -> ImageResult<Image> {
            let width = self.pixels.ncols();
            if iterations >= width {
                return Err(ImageError::InvalidSeamIterations {
                    iterations,
                    max: width.saturating_sub(1),
                    direction: SeamDirection::Vertical,
                });
            }

            let mut image = self.clone();
            let mut border = width;
            let mut energy_matrix: DMatrix<u32> =
                DMatrix::from_element(self.pixels.nrows(), self.pixels.ncols(), 0);
            for _ in 0..iterations {
                energy::calculate_vertical_energy_matrix(&image, &mut energy_matrix, border);
                let x = energy::calculate_min_energy_column(&energy_matrix, border);
                let seam = energy::calculate_optimal_vertical_path(&energy_matrix, border, x);
                image.carve_vertical_path(border, &seam);
                border -= 1;
            }
            image.crop(Rect {
                x1: 0,
                x2: border,
                y1: 0,
                y2: self.pixels.nrows(),
            })
        }

        fn seam_carve_horizontal(&self, iterations: usize) -> ImageResult<Image> {
            let height = self.pixels.nrows();
            if iterations >= height {
                return Err(ImageError::InvalidSeamIterations {
                    iterations,
                    max: height.saturating_sub(1),
                    direction: SeamDirection::Horizontal,
                });
            }

            let mut image = self.clone();
            let mut border = height;
            let mut energy_matrix: DMatrix<u32> =
                DMatrix::from_element(self.pixels.nrows(), self.pixels.ncols(), 0);
            for _ in 0..iterations {
                energy::calculate_horizontal_energy_matrix(&image, &mut energy_matrix, border);
                let y = energy::calculate_min_energy_row(&energy_matrix, border);
                let seam = energy::calculate_optimal_horizontal_path(&energy_matrix, border, y);
                image.carve_horizontal_path(border, &seam);
                border -= 1;
            }
            image.crop(Rect {
                x1: 0,
                x2: self.pixels.ncols(),
                y1: 0,
                y2: border,
            })
        }

        /// Carves a vertical path.
        ///
        /// # Parameters
        ///  `border` - the width up to which the energy matrix is calculated to
        ///  `seam` - the seam to carve
        fn carve_vertical_path(&mut self, border: usize, seam: &[usize]) {
            for (y, &col) in seam.iter().enumerate().take(self.pixels.nrows()) {
                for x in col..border - 1 {
                    self.pixels[(y, x)] = self.pixels[(y, x + 1)];
                }
            }
        }

        /// Carves a horizontal path.
        ///
        /// # Parameters
        ///  `border` - the height up to which the energy matrix is calculated to
        ///  `seam` - the seam to carve
        fn carve_horizontal_path(&mut self, border: usize, seam: &[usize]) {
            for (x, &row) in seam.iter().enumerate().take(self.pixels.ncols()) {
                for y in row..border - 1 {
                    self.pixels[(y, x)] = self.pixels[(y + 1, x)];
                }
            }
        }

        //=== IMAGE MANIPULATION ==================================================================

        /// Crop an image.
        pub fn crop(&self, rect: Rect) -> ImageResult<Image> {
            let width = self.pixels.ncols();
            let height = self.pixels.nrows();
            if rect.x1 >= rect.x2 || rect.y1 >= rect.y2 || rect.x2 > width || rect.y2 > height {
                return Err(ImageError::InvalidCrop {
                    x1: rect.x1,
                    x2: rect.x2,
                    y1: rect.y1,
                    y2: rect.y2,
                    width,
                    height,
                });
            }

            Ok(self.with_pixels(DMatrix::from_fn(
                rect.y2 - rect.y1,
                rect.x2 - rect.x1,
                |row, col| self.pixels[(rect.y1 + row, rect.x1 + col)],
            )))
        }

        /// Transposes an image.
        pub fn transpose(&self) -> Image {
            self.with_pixels(DMatrix::from_fn(
                self.pixels.ncols(),
                self.pixels.nrows(),
                |row, col| self.pixels[(col, row)],
            ))
        }

        /// Rotates an image clockwise.
        pub fn rotate(&self) -> Image {
            let height = self.pixels.nrows();
            self.with_pixels(DMatrix::from_fn(self.pixels.ncols(), height, |row, col| {
                self.pixels[(height - 1 - col, row)]
            }))
        }

        /// Inverts an image.
        pub fn invert(&self) -> Image {
            let scale = self.scale;
            self.with_pixels(DMatrix::from_fn(
                self.pixels.nrows(),
                self.pixels.ncols(),
                |row, col| {
                    let pixel = self.pixels[(row, col)];
                    Pixel {
                        red: scale - pixel.red,
                        green: scale - pixel.green,
                        blue: scale - pixel.blue,
                    }
                },
            ))
        }

        /// Mirror an image.
        pub fn mirror(&self) -> Image {
            let width = self.pixels.ncols();
            self.with_pixels(DMatrix::from_fn(self.pixels.nrows(), width, |row, col| {
                self.pixels[(row, width - 1 - col)]
            }))
        }

        /// Flood-fill a region from `(x, y)` using the supplied color.
        pub fn landfill(&self, coords: (usize, usize), rgb: (u8, u8, u8)) -> ImageResult<Image> {
            let (x, y) = coords;
            let width = self.pixels.ncols();
            let height = self.pixels.nrows();
            if x >= width || y >= height {
                return Err(ImageError::CoordinateOutOfBounds {
                    x,
                    y,
                    width,
                    height,
                });
            }

            let mut image = self.clone();
            let original = image.pixels[(y, x)];
            let replacement = Pixel {
                red: rgb.0,
                green: rgb.1,
                blue: rgb.2,
            };
            if original == replacement {
                return Ok(image);
            }

            let mut stack = vec![(x, y)];
            while let Some((x, y)) = stack.pop() {
                if image.pixels[(y, x)] != original {
                    continue;
                }
                image.pixels[(y, x)] = replacement;
                for (next_x, next_y) in Self::neighbors(x, y, width, height) {
                    if image.pixels[(next_y, next_x)] == original {
                        stack.push((next_x, next_y));
                    }
                }
            }

            Ok(image)
        }

        fn with_pixels(&self, pixels: DMatrix<Pixel>) -> Image {
            Image {
                magic_number: self.magic_number.clone(),
                scale: self.scale,
                pixels,
            }
        }

        fn neighbors(
            x: usize,
            y: usize,
            width: usize,
            height: usize,
        ) -> impl Iterator<Item = (usize, usize)> {
            let mut neighbors = Vec::with_capacity(4);
            if x > 0 {
                neighbors.push((x - 1, y));
            }
            if x + 1 < width {
                neighbors.push((x + 1, y));
            }
            if y > 0 {
                neighbors.push((x, y - 1));
            }
            if y + 1 < height {
                neighbors.push((x, y + 1));
            }
            neighbors.into_iter()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn pixel(red: u8, green: u8, blue: u8) -> Pixel {
            Pixel { red, green, blue }
        }

        fn image(width: usize, height: usize, pixels: Vec<Pixel>) -> Image {
            Image {
                magic_number: "P3".to_string(),
                scale: 255,
                pixels: DMatrix::from_fn(height, width, |row, col| pixels[row * width + col]),
            }
        }

        #[test]
        fn parses_p3_with_comments_and_arbitrary_whitespace() {
            let ppm = "P3\n# comment\n2 1\n255\n0 1 2   # pixel comment\n3 4 5\n";

            let image = Image::from_ppm_str(ppm).unwrap();

            assert_eq!(image.pixels.ncols(), 2);
            assert_eq!(image.pixels.nrows(), 1);
            assert_eq!(image.pixels[(0, 0)], pixel(0, 1, 2));
            assert_eq!(image.pixels[(0, 1)], pixel(3, 4, 5));
        }

        #[test]
        fn rejects_unsupported_maxval() {
            let ppm = "P3\n1 1\n256\n0 0 0\n";

            let err = Image::from_ppm_str(ppm).unwrap_err();

            assert!(matches!(err, ImageError::UnsupportedMaxval(256)));
        }

        #[test]
        fn rejects_wrong_sample_count() {
            let ppm = "P3\n1 1\n255\n0 0 0 1\n";

            let err = Image::from_ppm_str(ppm).unwrap_err();

            assert!(matches!(
                err,
                ImageError::InvalidSampleCount {
                    expected: 3,
                    actual: 4
                }
            ));
        }

        #[test]
        fn round_trips_written_ppm() {
            let image = image(2, 1, vec![pixel(10, 20, 30), pixel(40, 50, 60)]);
            let mut output = Vec::new();

            image.write_ppm(&mut output).unwrap();
            let parsed = Image::from_ppm_str(std::str::from_utf8(&output).unwrap()).unwrap();

            assert_eq!(parsed, image);
        }

        #[test]
        fn crops_to_requested_rectangle() {
            let image = image(
                3,
                2,
                vec![
                    pixel(1, 0, 0),
                    pixel(2, 0, 0),
                    pixel(3, 0, 0),
                    pixel(4, 0, 0),
                    pixel(5, 0, 0),
                    pixel(6, 0, 0),
                ],
            );

            let cropped = image
                .crop(Rect {
                    x1: 1,
                    x2: 3,
                    y1: 0,
                    y2: 2,
                })
                .unwrap();

            assert_eq!(cropped.pixels.ncols(), 2);
            assert_eq!(cropped.pixels.nrows(), 2);
            assert_eq!(cropped.pixels[(0, 0)], pixel(2, 0, 0));
            assert_eq!(cropped.pixels[(1, 1)], pixel(6, 0, 0));
        }

        #[test]
        fn rejects_invalid_crop_bounds() {
            let image = image(1, 1, vec![pixel(0, 0, 0)]);

            let err = image
                .crop(Rect {
                    x1: 0,
                    x2: 2,
                    y1: 0,
                    y2: 1,
                })
                .unwrap_err();

            assert!(matches!(err, ImageError::InvalidCrop { .. }));
        }

        #[test]
        fn invert_twice_returns_original_image() {
            let image = image(2, 1, vec![pixel(10, 20, 30), pixel(40, 50, 60)]);

            assert_eq!(image.invert().invert(), image);
        }

        #[test]
        fn landfill_handles_border_pixel() {
            let image = image(
                2,
                2,
                vec![
                    pixel(0, 0, 0),
                    pixel(255, 255, 255),
                    pixel(255, 255, 255),
                    pixel(255, 255, 255),
                ],
            );

            let filled = image.landfill((0, 0), (10, 20, 30)).unwrap();

            assert_eq!(filled.pixels[(0, 0)], pixel(10, 20, 30));
            assert_eq!(filled.pixels[(0, 1)], pixel(255, 255, 255));
            assert_eq!(filled.pixels[(1, 0)], pixel(255, 255, 255));
        }

        #[test]
        fn landfill_rejects_out_of_bounds_start() {
            let image = image(1, 1, vec![pixel(0, 0, 0)]);

            let err = image.landfill((1, 0), (10, 20, 30)).unwrap_err();

            assert!(matches!(err, ImageError::CoordinateOutOfBounds { .. }));
        }

        #[test]
        fn vertical_seam_carve_reduces_width() {
            let image = image(3, 3, vec![pixel(10, 20, 30); 9]);

            let carved = image.seam_carve(1, SeamDirection::Vertical).unwrap();

            assert_eq!(carved.pixels.ncols(), 2);
            assert_eq!(carved.pixels.nrows(), 3);
        }

        #[test]
        fn horizontal_seam_carve_reduces_height() {
            let image = image(3, 3, vec![pixel(10, 20, 30); 9]);

            let carved = image.seam_carve(1, SeamDirection::Horizontal).unwrap();

            assert_eq!(carved.pixels.ncols(), 3);
            assert_eq!(carved.pixels.nrows(), 2);
        }

        #[test]
        fn seam_carve_rejects_removing_entire_dimension() {
            let image = image(2, 2, vec![pixel(10, 20, 30); 4]);

            let err = image.seam_carve(2, SeamDirection::Vertical).unwrap_err();

            assert!(matches!(err, ImageError::InvalidSeamIterations { .. }));
        }
    }
}
