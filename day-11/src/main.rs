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
}

fn main() {
    println!("Hello, world!");
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
}
