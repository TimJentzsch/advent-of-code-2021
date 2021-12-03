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

/// Determine the gamma value from 
/// 
/// Gamma takes the most common bit and is the first value.
/// Epsilon takes the least common bit and is the second value.
fn determine_gamma_and_eplison(values: Vec<u16>, length: usize) -> (u16, u16) {
    let mut gamma: u16 = 0;
    let mut epsilon: u16 = 0;

    let majority = values.len() as u16 / 2;

    for i in 0..length {
        let one_count: u16 = values.iter().map(|value| {
            let bit = (value >> i) & 1;
            bit
        }).sum();

        // Determine the bit values
        let (gamma_bit, epsilon_bit): (u16, u16) = if one_count >= majority {
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

#[cfg(test)]
mod test {
    use crate::{parse_value_list, determine_gamma_and_eplison};

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
    fn should_determine_gamma_and_epsilon() {
        let values = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let expected = (22, 9);
        let actual = determine_gamma_and_eplison(values, 5);

        assert_eq!(actual, expected);
    }
}
