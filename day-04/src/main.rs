#[derive(Debug, PartialEq)]
struct Board<const R: usize, const C: usize> {
    /// The values of the bingo board.
    values: [[u16; R]; C],
    /// The mask of the bingo board, showing which values already occurred.
    mask: [[bool; R]; C],
}

impl<const R: usize, const C: usize> Board<{ R }, { C }> {
    /// Create a new board with the given values.
    fn new(values: [[u16; R]; C]) -> Board<R, C> {
        Board {
            values,
            mask: [[false; R]; C],
        }
    }

    /// Create a new board from a given board string.
    ///
    /// Example string:
    ///
    /// ```txt
    /// 22 13 17 11  0
    ///  8  2 23  4 24
    /// 21  9 14 16  7
    ///  6 10  3 18  5
    ///  1 12 20 15 19
    /// ```
    fn from_input(input: String) -> Board<R, C> {
        let mut values = [[0u16; R]; C];
        let row_strs: Vec<&str> = input.split("\n").into_iter().collect();

        // Go through all rows
        for row in 0..R {
            let col_str: Vec<&str> = row_strs[row].split_whitespace().into_iter().collect();

            // Go through all columns
            for col in 0..C {
                // Parse the value at the given position
                let val_str = col_str[col];
                let val: u16 = val_str.parse().unwrap();

                values[row][col] = val;
            }
        }

        Board::new(values)
    }

    /// Mark that the given value occurred.
    fn mark(&mut self, value: u16) {
        for row in 0..R {
            for col in 0..C {
                if self.values[row][col] == value {
                    self.mask[row][col] = true;
                }
            }
        }
    }

    /// Determines if this board has won, i.e. if a full row or column is marked.
    fn has_won(&self) -> bool {
        // Check rows
        for row in 0..R {
            let mut has_won = true;

            for col in 0..C {
                if !self.mask[row][col] {
                    has_won = false;
                    break;
                }
            }

            if has_won {
                return true;
            }
        }

        // Check rows
        for col in 0..R {
            let mut has_won = true;

            for row in 0..C {
                if !self.mask[row][col] {
                    has_won = false;
                    break;
                }
            }

            if has_won {
                return true;
            }
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}

/// Parse the given bingo numbers.
///
/// Example string:
///
/// `7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1`
fn parse_numbers(input: String) -> Vec<u16> {
    input
        .split(",")
        .into_iter()
        .map(|number_str| number_str.parse().unwrap())
        .collect()
}

/// Parse the given bingo input.
fn parse_input(input: String) -> (Vec<u16>, Vec<Board<5, 5>>) {
    let parts: Vec<&str> = input.split("\n\n").into_iter().collect();

    // The first part is the numbers
    let numbers_str = parts[0];
    // The rest of the parts are the boards
    let board_strs = &parts[1..];

    let numbers: Vec<u16> = parse_numbers(numbers_str.to_string());
    let boards = board_strs
        .iter()
        .map(|board_str| Board::from_input(board_str.to_string()))
        .collect();

    (numbers, boards)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, parse_numbers, Board};

    #[test]
    fn should_parse_board() {
        let input: String =
            "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19"
                .to_string();
        let expected: Board<5, 5> = Board::new([
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]);
        let actual = Board::from_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_numbers() {
        let input: String =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string();
        let expected: Vec<u16> = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let actual = parse_numbers(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_input() {
        let input: String =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19".to_string();
        let expected_numbers: Vec<u16> = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let expected_boards: Vec<Board<5, 5>> = vec![Board::new([
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ])];
        let (actual_numbers, actual_boards) = parse_input(input);

        assert_eq!(actual_numbers, expected_numbers);
        assert_eq!(actual_boards, expected_boards);
    }
}
