#[derive(Debug, PartialEq)]
struct Heightmap<const R: usize, const C: usize> {
    heights: [[u8; C]; R],
}

impl<const R: usize, const C: usize> Heightmap<R, C> {
    fn parse_row(input: String) -> [u8; C] {
        let mut row = [0u8; C];

        for (index, ch) in input.char_indices() {
            row[index] = ch.to_string().parse().unwrap();
        }

        row
    }

    fn from_input(input: String) -> Self {
        let mut heights = [[0u8; C]; R];

        for (col, line) in input.trim().split("\n").enumerate() {
            heights[col] = Self::parse_row(line.to_string());
        }

        Heightmap { heights }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Heightmap;

    #[test]
    fn should_parse_row() {
        let input = "2199943210".to_string();
        let expected = [2, 1, 9, 9, 9, 4, 3, 2, 1, 0];
        let actual = Heightmap::<1, 10>::parse_row(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_heightmap() {
        let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678".to_string();
        let expected = Heightmap {
            heights: [
                [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ],
        };
        let actual = Heightmap::<5, 10>::from_input(input);

        assert_eq!(actual, expected);
    }
}
