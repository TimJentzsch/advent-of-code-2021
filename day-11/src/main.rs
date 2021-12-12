use std::fs;

#[derive(Debug, PartialEq)]
struct OctopusGrid<const R: usize, const C: usize> {
    energy_levels: [[u8; C]; R],
}

impl<const R: usize, const C: usize> OctopusGrid<R, C> {
    fn new(energy_levels: [[u8; C]; R]) -> Self {
        Self { energy_levels }
    }

    fn parse_row(input: String) -> [u8; C] {
        let mut row = [0u8; C];

        for (i, ch) in input.trim().char_indices() {
            row[i] = ch.to_string().parse::<u8>().unwrap();
        }

        row
    }

    fn from_input(input: String) -> Self {
        let mut energy_levels = [[0u8; C]; R];

        for (i, row_str) in input.trim().split('\n').into_iter().enumerate() {
            energy_levels[i] = Self::parse_row(row_str.to_string());
        }

        Self { energy_levels }
    }

    /// Perform a single step.
    ///
    /// Returns the number of flashes that occurred.
    fn step(&mut self) -> u32 {
        let mut flashes = 0;

        // Increase energy level by 1
        for row in 0..R {
            for col in 0..C {
                self.energy_levels[row][col] += 1;
            }
        }

        // Flashes
        let mut check_flashes = true;

        while check_flashes {
            check_flashes = false;

            for row in 0..R {
                for col in 0..C {
                    if self.energy_levels[row][col] > 9 {
                        check_flashes = true;

                        self.energy_levels[row][col] = 0;
                        flashes += 1;

                        // Increase the energy level of adjacent octopuses
                        for dx in [-1, 0, 1] {
                            for dy in [-1, 0, 1] {
                                let (r, c) = (row as i32 + dx, col as i32 + dy);

                                if r >= 0
                                    && r < R as i32
                                    && c >= 0
                                    && c < C as i32
                                    // An octopus may only flash once per step
                                    && self.energy_levels[r as usize][c as usize] > 0
                                {
                                    self.energy_levels[r as usize][c as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        flashes
    }

    /// Perform multiple steps
    ///
    /// Returns the number of flashes that occurred.
    fn steps(&mut self, steps: usize) -> u32 {
        let mut flashes = 0;

        for _ in 0..steps {
            flashes += self.step();
        }

        flashes
    }
}

fn main() {
    // Read the input file
    let filename = "./input/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut grid = OctopusGrid::<10, 10>::from_input(input);
    let flashes = grid.steps(100);

    println!("The octopuses flashes {} times!", flashes);
}

#[cfg(test)]
mod tests {
    use crate::OctopusGrid;

    #[test]
    fn should_parse_row() {
        let input = "5483143223".to_string();
        let expected = [5, 4, 8, 3, 1, 4, 3, 2, 2, 3];
        let actual = OctopusGrid::<10, 10>::parse_row(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_input() {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526"
            .to_string();
        let expected = OctopusGrid::new([
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]);
        let actual = OctopusGrid::<10, 10>::from_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_perform_step() {
        let mut octopus_grid = OctopusGrid::<10, 10>::from_input(
            "6594254334
            3856965822
            6375667284
            7252447257
            7468496589
            5278635756
            3287952832
            7993992245
            5957959665
            6394862637"
                .to_string(),
        );
        let actual_flashes = octopus_grid.step();

        assert_eq!(actual_flashes, 35);

        let expected_grid = OctopusGrid::<10, 10>::from_input(
            "8807476555
            5089087054
            8597889608
            8485769600
            8700908800
            6600088989
            6800005943
            0000007456
            9000000876
            8700006848"
                .to_string(),
        );

        assert_eq!(octopus_grid, expected_grid);
    }

    #[test]
    fn should_perform_multiple_steps() {
        let mut octopus_grid = OctopusGrid::<10, 10>::from_input(
            "5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526"
                .to_string(),
        );
        let actual_flashes = octopus_grid.steps(100);

        assert_eq!(actual_flashes, 1656);

        let expected_grid = OctopusGrid::<10, 10>::from_input(
            "0397666866
            0749766918
            0053976933
            0004297822
            0004229892
            0053222877
            0532222966
            9322228966
            7922286866
            6789998766"
                .to_string(),
        );

        assert_eq!(octopus_grid, expected_grid);
    }
}
