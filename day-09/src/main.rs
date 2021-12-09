struct Heightmap<const R: usize, const C: usize> {
    heights: [[u8; R]; C],
}

impl<const R: usize, const C: usize> Heightmap<R, C> {
    fn parse_row(input: String) -> [u8; R] {
        let mut row = [0u8; R];

        for (index, ch) in input.char_indices() {
            row[index] = ch.to_string().parse().unwrap();
        }

        row
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
        let actual = Heightmap::<10, 1>::parse_row(input);

        assert_eq!(actual, expected);
    }
}
