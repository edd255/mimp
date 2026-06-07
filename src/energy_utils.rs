//! Seam Carving uses color differences of neighboring pixels as dispensability score. This
//! difference is called energy. This crate contains methods to calculate the energy of an image
//! and to find the optimal path according to this dispensability score.

pub mod energy {
    use crate::image_utils::image::Image;
    use crate::pixel_utils::pixel::Pixel;
    use nalgebra::DMatrix;
    use std::cmp::min;

    /// Pixels have local energy which is the sum of the color differences of the current pixel and
    /// its left and upper neighbor (if present). The total energy of a pixel is calculated by
    /// adding the minimum of the total energy of the three pixels above the current pixels.
    ///
    /// # Parameters
    ///  `image` - the pixel matrix
    ///  `energy` - the allocated energy matrix
    ///  `border` - the width up to which column in the image the energy should be calculated
    pub fn calculate_vertical_energy_matrix(
        image: &Image,
        energy: &mut DMatrix<u32>,
        border: usize,
    ) {
        // Calculation of local energy
        // Edge Case: First Element
        energy[(0, 0)] = 0;
        // Edge Case: First Row
        for j in 1..border {
            let current = (0, j);
            let left = (0, j - 1);
            energy[current] = Pixel::color_diff(image.pixels[current], image.pixels[left]);
        }
        // Edge Case: Left Border
        for i in 1..image.pixels.nrows() {
            let current = (i, 0);
            let above = (i - 1, 0);
            energy[current] = Pixel::color_diff(image.pixels[current], image.pixels[above]);
        }
        // No Edge Cases
        for i in 1..image.pixels.nrows() {
            for j in 1..border {
                let current = (i, j);
                let left = (i, j - 1);
                let above = (i - 1, j);
                energy[current] = Pixel::color_diff(image.pixels[current], image.pixels[left])
                    + Pixel::color_diff(image.pixels[current], image.pixels[above]);
            }
        }
        // Calculation of total energy
        for i in 1..image.pixels.nrows() {
            for j in 0..border {
                let current = (i, j);
                let mut minimum = energy[(i - 1, j)];
                if j > 0 {
                    minimum = min(minimum, energy[(i - 1, j - 1)]);
                }
                if j + 1 < border {
                    minimum = min(minimum, energy[(i - 1, j + 1)]);
                }
                energy[current] += minimum;
            }
        }
    }

    /// Pixels have local energy which is the sum of the color differences of the current pixel and
    /// its left and lower neighbor (if present). The total energy of a pixel is calculated by
    /// adding the minimum of the total energy of the three pixels left to the current pixel.
    ///
    /// # Parameters
    ///  `image` - the pixel matrix
    ///  `energy` - the allocated energy matrix
    ///  `border` - the height up to which row in the image the energy should be calculated
    pub fn calculate_horizontal_energy_matrix(
        image: &Image,
        energy: &mut DMatrix<u32>,
        border: usize,
    ) {
        // Calculation of local energy
        // Edge Case: First Element
        energy[(0, 0)] = 0;
        // Edge Case: First Column
        for j in 1..border {
            let current = (j, 0);
            let left = (j - 1, 0);
            energy[current] = Pixel::color_diff(image.pixels[current], image.pixels[left]);
        }
        // Edge Case: First Row
        for i in 1..image.pixels.ncols() {
            let current = (0, i);
            let lower = (0, i - 1);
            energy[current] = Pixel::color_diff(image.pixels[current], image.pixels[lower]);
        }
        // No Edge Cases
        for i in 1..image.pixels.ncols() {
            for j in 1..border {
                let current = (j, i);
                let left = (j - 1, i);
                let lower = (j, i - 1);
                energy[current] = Pixel::color_diff(image.pixels[current], image.pixels[left])
                    + Pixel::color_diff(image.pixels[current], image.pixels[lower]);
            }
        }
        // Calculation of total energy
        for i in 1..image.pixels.ncols() {
            for j in 0..border {
                let current = (j, i);
                let mut minimum = energy[(j, i - 1)];
                if j > 0 {
                    minimum = min(minimum, energy[(j - 1, i - 1)]);
                }
                if j + 1 < border {
                    minimum = min(minimum, energy[(j + 1, i - 1)]);
                }
                energy[current] += minimum;
            }
        }
    }

    /// Finds the column at the row `border` with the smallest energy.
    pub fn calculate_min_energy_column(energy: &DMatrix<u32>, border: usize) -> usize {
        let mut column: usize = 0;
        for i in 1..border {
            if energy[(energy.nrows() - 1, column)] > energy[(energy.nrows() - 1, i)] {
                column = i;
            }
        }
        column
    }

    /// Finds the row at the column `border` with the smallest energy.
    pub fn calculate_min_energy_row(energy: &DMatrix<u32>, border: usize) -> usize {
        let mut row: usize = 0;
        for i in 1..border {
            if energy[(row, energy.ncols() - 1)] > energy[(i, energy.ncols() - 1)] {
                row = i;
            }
        }
        row
    }

    /// The optimal seam is a pixel path with minimal total energy. The pixel in the bottom most
    /// row with the minimal total energy is the start pixel. If there are multiple optimal
    /// neighbors with distinct positions in the last row, the neighbor with the lowest x-coordinate
    /// is used. If a pixel has multiple optimal neighbors, the top center neighbor, and then the
    /// top left neighbor is preferred.
    ///
    /// # Parameters
    ///  `energy` - the allocated energy matrix
    ///  `border` - the width up to which column in the image the energy should be calculated
    ///  `start` - the pixel with the minimal energy
    ///
    /// # Return
    ///  the vertical seam
    pub fn calculate_optimal_vertical_path(
        energy: &DMatrix<u32>,
        border: usize,
        start: usize,
    ) -> Vec<usize> {
        let mut seam = vec![0; energy.nrows()];
        seam[energy.nrows() - 1] = start;
        for j in (1..energy.nrows()).rev() {
            let mut best = seam[j];
            let mut minimum = energy[(j - 1, best)];
            if seam[j] > 0 && energy[(j - 1, seam[j] - 1)] < minimum {
                best = seam[j] - 1;
                minimum = energy[(j - 1, best)];
            }
            if seam[j] + 1 < border && energy[(j - 1, seam[j] + 1)] < minimum {
                best = seam[j] + 1;
            }
            seam[j - 1] = best;
        }
        seam
    }

    /// The optimal seam is a pixel path with minimal total energy. The pixel in the rightmost
    /// column with the minimal total energy is the start pixel. If there are multiple optimal
    /// neighbors with distinct positions in the last column, the neighbor with the lowest y-coordinate
    /// is used. If a pixel has multiple optimal neighbors, the left center neighbor, and then the
    /// top left neighbor is preferred.
    ///
    /// # Parameters
    ///  `energy` - the allocated energy matrix
    ///  `border` - the height up to which row in the image the energy should be calculated
    ///  'start' - the pixel with the minimal energy
    ///
    /// # Return
    ///  the horizontal seam
    pub fn calculate_optimal_horizontal_path(
        energy: &DMatrix<u32>,
        border: usize,
        start: usize,
    ) -> Vec<usize> {
        let mut seam = vec![0; energy.ncols()];
        seam[energy.ncols() - 1] = start;
        for j in (1..energy.ncols()).rev() {
            let mut best = seam[j];
            let mut minimum = energy[(best, j - 1)];
            if seam[j] > 0 && energy[(seam[j] - 1, j - 1)] < minimum {
                best = seam[j] - 1;
                minimum = energy[(best, j - 1)];
            }
            if seam[j] + 1 < border && energy[(seam[j] + 1, j - 1)] < minimum {
                best = seam[j] + 1;
            }
            seam[j - 1] = best;
        }
        seam
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::pixel_utils::pixel::Pixel;

        fn image(width: usize, height: usize) -> Image {
            Image {
                magic_number: "P3".to_string(),
                scale: 255,
                pixels: DMatrix::from_element(
                    height,
                    width,
                    Pixel {
                        red: 10,
                        green: 20,
                        blue: 30,
                    },
                ),
            }
        }

        #[test]
        fn vertical_energy_handles_left_border_without_underflow() {
            let image = image(3, 3);
            let mut energy = DMatrix::from_element(3, 3, 0);

            calculate_vertical_energy_matrix(&image, &mut energy, 3);

            assert_eq!(energy[(2, 0)], 0);
        }

        #[test]
        fn horizontal_energy_handles_top_border_without_underflow() {
            let image = image(3, 3);
            let mut energy = DMatrix::from_element(3, 3, 0);

            calculate_horizontal_energy_matrix(&image, &mut energy, 3);

            assert_eq!(energy[(0, 2)], 0);
        }

        #[test]
        fn vertical_path_handles_left_border_without_underflow() {
            let energy = DMatrix::from_element(3, 3, 1);

            let seam = calculate_optimal_vertical_path(&energy, 3, 0);

            assert_eq!(seam, vec![0, 0, 0]);
        }

        #[test]
        fn horizontal_path_handles_top_border_without_underflow() {
            let energy = DMatrix::from_element(3, 3, 1);

            let seam = calculate_optimal_horizontal_path(&energy, 3, 0);

            assert_eq!(seam, vec![0, 0, 0]);
        }
    }
}
