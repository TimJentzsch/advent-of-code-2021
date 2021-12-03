fn main() {
    println!("Hello, world!");
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

#[cfg(test)]
mod test {
    use crate::parse_value_list;

    #[test]
    fn should_parse_value_list() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
                .to_string();
        let expected = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let actual = parse_value_list(input);

        assert_eq!(actual, expected);
    }
}
