pub mod pixel {
    pub struct Pixel {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl Pixel {
        pub fn color_diff(&self, other: &Pixel) -> i16 {
            let red_diff: i16 = i16::from(self.red) - i16::from(other.red);
            let green_diff: i16 =
                i16::from(self.green) - i16::from(other.green);
            let blue_diff: i16 = i16::from(self.blue) - i16::from(other.blue);

            Self::square(red_diff)
                + Self::square(green_diff)
                + Self::square(blue_diff)
        }

        pub fn invert(&mut self) {
            self.red = 255 - self.red;
            self.green = 255 - self.green;
            self.blue = 255 - self.blue;
        }

        fn square(a: i16) -> i16 {
            a * a
        }
    }
}
