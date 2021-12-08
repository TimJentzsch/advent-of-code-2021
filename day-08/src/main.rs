#[derive(Debug)]
struct InvalidSegment;

trait Digit {
    fn from_input(input: String) -> Self;
    fn from_segment(segment: char) -> Result<Self, InvalidSegment>
    where
        Self: Sized;
    fn segment_count(&self) -> u8;
}

impl Digit for u8 {
    /// Convert a segment string to a digit.
    fn from_input(input: String) -> Self {
        input
            .chars()
            .map(|seg| u8::from_segment(seg))
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            // Combine the segments to the digit representation
            .sum()
    }

    /// Convert the segment to a part of the digit.
    fn from_segment(segment: char) -> Result<Self, InvalidSegment> {
        match segment {
            'a' => Ok(0b1000000),
            'b' => Ok(0b0100000),
            'c' => Ok(0b0010000),
            'd' => Ok(0b0001000),
            'e' => Ok(0b0000100),
            'f' => Ok(0b0000010),
            'g' => Ok(0b0000001),
            _ => Err(InvalidSegment),
        }
    }

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

    #[test]
    fn should_parse_digit_input() {
        let input = "abdfg".to_string();
        let expected: u8 = 0b1101011;
        let actual = u8::from_input(input);

        assert_eq!(actual, expected);
    }
}
