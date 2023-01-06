pub mod image {
    use crate::pixel_utils::pixel::Pixel;
    use std::fs;

    pub struct Image {
        pub width: usize,
        pub height: usize,
        pub magic_number: String,
        pub scale: u8,
        pub pixels: Vec<Pixel>,
    }

    impl Image {
        /**
         * Source:
         *   https://github.com/chris-paterson/PPM
         */
        pub fn read_file(filename: String) -> Image {
            let contents = match fs::read_to_string(filename) {
                Ok(str) => str,
                Err(err) => panic!("{:?}", err),
            };
            let lines: Vec<&str> = contents.split('\n').collect();
            let header: Vec<&str> = lines[0..3].to_vec();
            let body: Vec<&str> = lines[3..].to_vec();
            let (magic_number, width, height, scale) = match Self::parse_header(&header) {
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
         * Source:
         *   https://github.com/chris-paterson/PPM
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
         * Source:
         *   https://github.com/chris-paterson/PPM
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

        fn brightness(&self) -> u32 {
            let num: u32 = (self.width * self.height).try_into().unwrap();
            let mut sum: u32 = 0;

            for pixel in &self.pixels {
                sum += ((pixel.red as u32) + (pixel.green as u32) + (pixel.blue as u32)) / 3;
            }

            sum / num
        }

        pub fn statistics(&self) {
            println!("Type:       {}", self.magic_number);
            println!("Height:     {}", self.height);
            println!("Width:      {}", self.width);
            println!("Pixels:     {}", self.pixels.len());
            println!("Brightness: {}", self.brightness());
        }
    }
}
