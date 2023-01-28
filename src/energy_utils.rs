use crate::image_utils::image::Image;
use crate::pixel_utils::pixel::Pixel;
use nalgebra::DMatrix;

pub fn calculate_energy(
    image: Image,
    border: usize,
) -> DMatrix<i16> {
    let mut energy: DMatrix<i16> = DMatrix::from_element(image.pixels.nrows(), image.pixels.ncols(), 0);

    // Edge Case: First Element
    energy[(0, 0)] = 0;

    // Edge Case: First Row
    for j in 1..border {
        let current = image.pixels[(0, j)];
        let left = image.pixels[(0, j - 1)];

        energy[(0, j)] = Pixel::color_diff(&current, &left);
    }

    // Edge Case: Left Border
    for i in 1..image.pixels.nrows() {
        let current = image.pixels[(i, 0)];
        let above = image.pixels[(i - 1, 0)];

        energy[(i, 0)] = Pixel::color_diff(&current, &above);
    }

    // No Edge Cases
    for i in 1..image.pixels.nrows() {
        for j in 1..border {
            let current = image.pixels[(i, j)];
            let left = image.pixels[(i, j - 1)];
            let above = image.pixels[(i - 1, j)];

            energy[(i, j)] = Pixel::color_diff(&current, &left) + Pixel::color_diff(&left, &above);
        }
    }

    // First Row remains unchanged (no upper neighbors)
    for i in 1..image.pixels.nrows() {
        let mut current = image.pixels[(i, 0)];
        let mut above = image.pixels[(i - 1, 0)];
        let mut right = image.pixels[(i - 1, 1)];
        let mut left;

        // Edge Case: Left Border
        energy[(i, 0)] += energy[(i - 1, 0)].min(energy[(i - 1, 1)]);

        // No Edge Cases
        for j in 1..border - 1 {
            current = image.pixels[(i, j)];
            left = image.pixels[(i - 1, j - 1)];
            above = image.pixels[(i - 1, j)];
            right = image.pixels[(i - 1, j + 1)];

            energy[(i, j)] =
                energy[(i - 1, j)].min(energy[(i - 1, j - 1)].min(energy[(i - 1, j + 1)]));
        }

        // Edge Case: Right Border
        current = image.pixels[(i, border - 1)];
        above = image.pixels[(i - 1, border - 1)];
        left = image.pixels[(i - 1, border - 2)];

        energy[(i, border - 1)] += energy[(i - 1, border - 2)].min(energy[(i - 1, border - 1)]);
    }

    energy
}
