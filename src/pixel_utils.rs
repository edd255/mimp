pub mod pixel {
    use num_traits::Zero;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Pixel {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl Pixel {
        /// Computes color differences between two pixels, by subtracting their values and squaring
        /// them.
        #[allow(clippy::cast_sign_loss)]
        pub fn color_diff(pixel1: Pixel, pixel2: Pixel) -> u32 {
            let red_diff = i32::from(pixel1.red) - i32::from(pixel2.red);
            let green_diff = i32::from(pixel1.green) - i32::from(pixel2.green);
            let blue_diff = i32::from(pixel1.blue) - i32::from(pixel2.blue);
            let red_diff_squared = red_diff * red_diff;
            let green_diff_squared = green_diff * green_diff;
            let blue_diff_squared = blue_diff * blue_diff;
            (red_diff_squared + green_diff_squared + blue_diff_squared) as u32
        }

        /// Inverts the colors of a pixel.
        pub fn invert(&mut self) {
            self.red = 255 - self.red;
            self.green = 255 - self.green;
            self.blue = 255 - self.blue;
        }
    }

    /// Implements the Zero trait for Pixel.
    impl Zero for Pixel {
        /// Returns a pixel with zero values for rgb colors.
        fn zero() -> Self {
            Self {
                red: 0,
                green: 0,
                blue: 0,
            }
        }

        /// Returns true if the pixel colors are only zero.
        fn is_zero(&self) -> bool {
            self.red == 0 && self.green == 0 && self.blue == 0
        }
    }

    /// Implements the Add trait for Pixel.
    impl std::ops::Add for Pixel {
        type Output = Self;

        /// Adds the colors of other to self.
        fn add(self, other: Self) -> Self {
            Self {
                red: self.red.saturating_add(other.red),
                green: self.green.saturating_add(other.green),
                blue: self.blue.saturating_add(other.blue),
            }
        }
    }
}
