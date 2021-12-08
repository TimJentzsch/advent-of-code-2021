trait Digit {
    fn segment_count(&self) -> u8;
}

impl Digit for u8 {
    /// Count the number of activated segments
    fn segment_count(&self) -> u8 {
        (0..8).into_iter().map(|index| (self >> index) & 1).sum()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Digit;

    #[test]
    fn should_count_active_segments() {
        let digit: u8 = 0b1011011;
        let actual = digit.segment_count();

        assert_eq!(actual, 5);
    }
}
