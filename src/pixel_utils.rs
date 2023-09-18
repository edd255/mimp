pub mod pixel {
    #[derive(Clone, PartialEq, Debug)]
    pub struct Pixel {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl Pixel {
        pub fn color_diff(pixel1: &Pixel, pixel2: &Pixel) -> i16 {
            let red_diff: i16 = i16::from(pixel1.red) - i16::from(pixel2.red);
            let green_diff: i16 = i16::from(pixel1.green) - i16::from(pixel2.green);
            let blue_diff: i16 = i16::from(pixel1.blue) - i16::from(pixel2.blue);

            Self::square(red_diff) + Self::square(green_diff) + Self::square(blue_diff)
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
