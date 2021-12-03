use std::fs;

fn main() {
    // Read the input file
    let filename = "./input/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let values = parse_value_list(input);
    let (gamma, epsilon) = determine_gamma_and_eplison(values, 12);
    let power_consumption = gamma as u32 * epsilon as u32;

    println!("The power consumption is {}", power_consumption);
}

/// Parse the given input to commands.
fn parse_value_list(input: String) -> Vec<u16> {
    input
        .split('\n')
        .into_iter()
        // Parse the binary number to an integer
        .map(|line| u16::from_str_radix(line, 2))
        // Filter out invalid lines
        .filter(|parse_result| match parse_result {
            Ok(_) => true,
            Err(_) => false,
        })
        // Unwrap the result
        .map(|parse_result| parse_result.unwrap())
        .collect()
}

/// Get the bit at the given position (from the right).
fn get_bit_at_position(value: u16, position: usize) -> u16 {
    (value >> position) & 1
}

/// Determine the most common bit at the given position (from the right).
fn determine_most_common_bit(values: &Vec<u16>, position: usize) -> u16 {
    let one_count: u16 = values
        .iter()
        .map(|value| get_bit_at_position(*value, position))
        .sum();

    // Determine the most common bit
    if one_count >= (values.len() as u16 - one_count) {
        1
    } else {
        0
    }
}

/// Determine the gamma value from
///
/// Gamma takes the most common bit and is the first value.
/// Epsilon takes the least common bit and is the second value.
fn determine_gamma_and_eplison(values: Vec<u16>, length: usize) -> (u16, u16) {
    let mut gamma: u16 = 0;
    let mut epsilon: u16 = 0;

    for i in 0..length {
        // Determine the bit values
        let (gamma_bit, epsilon_bit): (u16, u16) = if determine_most_common_bit(&values, i) == 1 {
            (1, 0)
        } else {
            (0, 1)
        };

        // Add the bits to the numbers
        gamma += gamma_bit << i;
        epsilon += epsilon_bit << i;
    }

    (gamma, epsilon)
}

/// Determine the oxygen generator rating.
fn determine_oxygen_generator_rating(values: Vec<u16>, length: usize) -> u16 {
    let mut values = values;

    for i in (0..length).rev() {
        let most_common_bit = determine_most_common_bit(&values, i);

        values = values
            .iter()
            .filter(|value| {
                let bit = get_bit_at_position(**value, i);
                bit == most_common_bit
            })
            .map(|value| *value)
            .collect();
    }

    *values.first().unwrap()
}

/// Determine the C02 scrubber rating.
fn determine_co2_scrubber_rating(values: Vec<u16>, length: usize) -> u16 {
    let mut values = values;

    for i in (0..length).rev() {
        let most_common_bit = determine_most_common_bit(&values, i);

        values = values
            .iter()
            .filter(|value| {
                let bit = get_bit_at_position(**value, i);
                bit != most_common_bit
            })
            .map(|value| *value)
            .collect();

        println!("MCB: {}, Len: {}", most_common_bit, values.len());
    }

    *values.first().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{
        determine_co2_scrubber_rating, determine_gamma_and_eplison, determine_most_common_bit,
        determine_oxygen_generator_rating, parse_value_list,
    };

    #[test]
    fn should_parse_value_list() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
                .to_string();
        let expected = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let actual = parse_value_list(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_most_common_bit_single_value_0() {
        let values = vec![0];
        let expected = 0;
        let actual = determine_most_common_bit(&values, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_most_common_bit_single_value_1() {
        let values = vec![1];
        let expected = 1;
        let actual = determine_most_common_bit(&values, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_most_common_bit_multiple_values_0() {
        let values = vec![
            0b11110, 0b10110, 0b10111, 0b10101, 0b11100, 0b10000, 0b11001,
        ];
        let expected = 0;
        let actual = determine_most_common_bit(&values, 3);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_gamma_and_epsilon() {
        let values = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let expected = (22, 9);
        let actual = determine_gamma_and_eplison(values, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_oxygen_generator_rating() {
        let values = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let expected = 23;
        let actual = determine_oxygen_generator_rating(values, 5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_c02_scrubber_rating() {
        let values = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let expected = 23;
        let actual = determine_co2_scrubber_rating(values, 10);

        assert_eq!(actual, expected);
    }
}
