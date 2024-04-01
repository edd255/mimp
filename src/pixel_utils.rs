pub mod pixel {
    use num_traits::Zero;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Pixel {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl Pixel {
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

        pub fn invert(&mut self) {
            self.red = 255 - self.red;
            self.green = 255 - self.green;
            self.blue = 255 - self.blue;
        }
    }

    impl Zero for Pixel {
        fn zero() -> Self {
            Self {
                red: 0,
                green: 0,
                blue: 0,
            }
        }

        fn is_zero(&self) -> bool {
            self.red == 0 && self.green == 0 && self.blue == 0
        }
    }

    impl std::ops::Add for Pixel {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                red: self.red.saturating_add(other.red),
                green: self.green.saturating_add(other.green),
                blue: self.blue.saturating_add(other.blue),
            }
        }
    }
}
