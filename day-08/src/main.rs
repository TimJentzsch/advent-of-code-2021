use std::fs;

#[derive(Debug)]
struct InvalidSegment;

trait Digit {
    fn from_input(input: String) -> Self;
    fn from_segment(segment: char) -> Result<Self, InvalidSegment>
    where
        Self: Sized;
    fn segment_count(&self) -> u8;
    fn remap(&self, map: &[u8; 7]) -> Self;
    fn value(&self) -> u8;
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

    /// Remap the segments.
    fn remap(&self, map: &[u8; 7]) -> Self {
        let mut remapped_digit = 0u8;

        for index in 0..7 {
            let bit = (self >> (6-index)) & 1;
            if bit == 1 {
                remapped_digit += map[index];
            }
        }

        remapped_digit
    }

    /// The decimal value of the digit.
    /// Only works after it has been converted to the default format.
    fn value(&self) -> u8 {
        match self {
            0b1110111 => 0,
            0b0010010 => 1,
            0b1011101 => 2,
            0b1011011 => 3,
            0b0111010 => 4,
            0b1101011 => 5,
            0b1101111 => 6,
            0b1010010 => 7,
            0b1111111 => 8,
            0b1111011 => 9,
            _ => panic!("Invalid digit value {:#07b}", self),
        }
    }
}

fn main() {
    let filename = "./input/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = parse_input(input);

    let mut unique_digit_count = 0;

    for (_, output_digits) in lines {
        unique_digit_count += output_digits
            .into_iter()
            .map(|digit| digit.segment_count())
            .filter(|count| *count == 2 || *count == 3 || *count == 4 || *count == 7)
            .count();
    }

    println!("Number of unique output digits: {}", unique_digit_count);
}

/// Parse a list of digits.
///
/// Example input:
///
/// `eacfd acdfbe cbdegf fcbaedg`
fn parse_digit_list(input: String) -> Vec<u8> {
    input
        .split(" ")
        .into_iter()
        .map(|segments| u8::from_input(segments.to_string()))
        .collect()
}

/// Parse one line of input to the input and output digits.
///
/// Example input:
///
/// `badc bd dbeaf cfdbge dfb cfbdea efbag edcfgab dcafe degfca | eacfd acdfbe cbdegf fcbaedg`
fn parse_input_line(input: String) -> (Vec<u8>, Vec<u8>) {
    let mut parts = input.trim().split(" | ");

    let input = parse_digit_list(parts.next().unwrap().to_string());
    let output = parse_digit_list(parts.next().unwrap().to_string());

    (input, output)
}

/// Parse the input file.
fn parse_input(input: String) -> Vec<(Vec<u8>, Vec<u8>)> {
    input
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| parse_input_line(line.to_string()))
        .collect()
}

fn filter_digits_with_count(digits: &Vec<u8>, count: u8) -> Vec<u8> {
    digits
        .into_iter()
        .filter(|digit| digit.segment_count() == count)
        .map(|&digit| digit)
        .collect()
}

fn find_digit_with_count(digits: &Vec<u8>, count: u8) -> u8 {
    *filter_digits_with_count(digits, count).first().unwrap()
}

fn calculate_output_value(input: String) -> u64 {
    let (input_digits, output_digits) = parse_input_line(input);
    let mapping = determine_mapping(input_digits);

    let mut value = 0;

    for i in 0..4 {
        let digit_value = output_digits[i].remap(&mapping).value() as u64;
        println!("Value: {}", digit_value);
        value += digit_value * 10u64.pow((3 - i).try_into().unwrap());
    }

    value
}

/// Determine the mapping of the segments.
fn determine_mapping(digits: Vec<u8>) -> [u8; 7] {
    // Unique numbers
    // One
    let cf = find_digit_with_count(&digits, 2);
    // Four
    let bcdf = find_digit_with_count(&digits, 4);
    // Seven
    let acf = find_digit_with_count(&digits, 3);
    // Eight
    let abcdefg = find_digit_with_count(&digits, 7);

    let a = acf - cf;
    let bd = bcdf - cf;

    // Common segments of digits with five segments
    let adg = filter_digits_with_count(&digits, 5)
        .into_iter()
        .reduce(|a, b| a & b)
        .unwrap();

    // Common segments of digits with six segments
    let abfg = filter_digits_with_count(&digits, 6)
        .into_iter()
        .reduce(|a, b| a & b)
        .unwrap();

    let bdf = adg ^ abfg;
    let f = bdf - bd;

    let c = cf - f;

    // Two
    let acdeg = filter_digits_with_count(&digits, 5)
        .into_iter()
        .filter(|&digit| digit & f == 0)
        .next()
        .unwrap();
    
    let b = abcdefg - acdeg - f;
    let d = bcdf - cf - b;

    let g = adg - a - d;
    let e = abcdefg - a - b - c - d - f - g;

    [a, b, c, d, e, f, g]
}

#[cfg(test)]
mod tests {
    use crate::{parse_digit_list, Digit, calculate_output_value};

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

    #[test]
    fn should_parse_digit_list() {
        let input = "eacfd acdfbe cbdegf fcbaedg".to_string();
        let expected: Vec<u8> = vec![0b1011110, 0b1111110, 0b0111111, 0b1111111];
        let actual = parse_digit_list(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_remap_value() {
        let input = 0b1101010;
        // cgafedb -> abcdefg
        let mapping = [0b0010000, 0b0000001, 0b1000000, 0b0000010, 0b0000100, 0b0001000, 0b0100000];
        let expected = 0b0011011;
        
        let actual = input.remap(&mapping);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_calculate_output_value() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string();
        let expected = 5353u64;
        let actual = calculate_output_value(input);

        assert_eq!(actual, expected);
    }
}
