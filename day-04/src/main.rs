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

#[cfg(test)]
mod tests {
    use crate::Board;

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
}
