struct Board<const R: usize, const C: usize> {
    /// The values of the bingo board.
    values: [[u16; R]; C],
    /// The mask of the bingo board, showing which values already occurred.
    mask: [[bool; R]; C],
}

impl<const R: usize, const C: usize> Board<{R}, {C}> {
    /// Create a new board with the given values.
    fn new(values: [[u16; R]; C]) -> Board<R, C> {
        Board { values, mask: [[false; R]; C] }
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
