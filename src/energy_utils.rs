pub mod energy {
    use crate::image_utils::image::Image;
    use crate::pixel_utils::pixel::Pixel;
    use nalgebra::DMatrix;

    pub fn calculate_energy(image: &Image, border: usize) -> DMatrix<i16> {
        let mut energy: DMatrix<i16> =
            DMatrix::from_element(image.pixels.nrows(), image.pixels.ncols(), 0);

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
                    + Pixel::color_diff(image.pixels[left], image.pixels[above]);
            }
        }

        // First Row remains unchanged (no upper neighbors)
        for i in 1..image.pixels.nrows() {
            let mut current = (i, 0);
            let mut above = (i - 1, 0);
            let mut right = (i - 1, 1);
            let mut left;

            // Edge Case: Left Border
            energy[current] += energy[above].min(energy[right]);

            // No Edge Cases
            for j in 1..border - 1 {
                current = (i, j);
                left = (i - 1, j - 1);
                above = (i - 1, j);
                right = (i - 1, j + 1);

                energy[current] = energy[above].min(energy[left].min(energy[right]));
            }

            // Edge Case: Right Border
            current = (i, border - 1);
            above = (i - 1, border - 1);
            left = (i - 1, border - 2);

            energy[current] += energy[left].min(energy[above]);
        }

        energy
    }

    pub fn calculate_min_energy_column(energy: &DMatrix<i16>, border: usize) -> usize {
        let mut column: usize = 0;
        for i in 1..border {
            if energy[(energy.nrows() - 1, column)] > energy[(energy.nrows() - 1, i)] {
                column = i;
            }
        }
        column
    }

    pub fn calculate_optimal_path(energy: &DMatrix<i16>, border: usize, start: usize) -> Vec<usize> {
        let mut seam = Vec::with_capacity(energy.nrows());
        seam[energy.nrows() - 1] = start;
        for j in energy.nrows()..0 {
            let left = (j - 1, seam[j] - 1);
            let above = (j - 1, seam[j]);
            let right = (j - 1, seam[j] + 1);

            if seam[j] == 0 {
                // Case: Left border

                if energy[above] <= energy[right] {
                    seam[j - 1] = seam[j];
                } else {
                    seam[j - 1] = seam[j] + 1;
                }
            } else if seam[j] == border - 1 {
                // Case: Right Border

                if energy[above] <= energy[left] {
                    seam[j - 1] = seam[j];
                } else {
                    seam[j - 1] = seam[j] - 1;
                }
            } else if energy[above] == energy[left] {
                // Precedence for multiple optimal pixels

                if energy[above] <= energy[right] {
                    seam[j - 1] = seam[j];
                } else {
                    seam[j - 1] = seam[j] + 1;
                }
            } else if energy[above] <= energy[right] {
                if energy[above] <= energy[left] {
                    seam[j - 1] = seam[j];
                } else {
                    seam[j - 1] = seam[j] - 1;
                }
            } else if energy[above] > energy[left] && energy[left] <= energy[right] {
                seam[j - 1] = seam[j] - 1;
            } else if energy[left] > energy[above] && energy[above] <= energy[right] {
                seam[j - 1] = seam[j];
            } else {
                seam[j - 1] = seam[j] + 1;
            }
        }
        seam
    }
}
