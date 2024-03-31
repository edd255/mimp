pub mod pixel {
    use num_traits::Zero;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Pixel {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl Pixel {
        pub fn color_diff(pixel1: &Pixel, pixel2: &Pixel) -> u32 {
            let red_diff = pixel1.red as i32 - pixel2.red as i32;
            let green_diff = pixel1.green as i32 - pixel2.green as i32;
            let blue_diff = pixel1.blue as i32 - pixel2.blue as i32;
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
