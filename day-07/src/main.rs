fn main() {
    println!("Hello, world!");
}

/// Parse the list of crab positions
fn parse_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .into_iter()
        .map(|value_str| value_str.parse::<usize>())
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn should_parse_input() {
        let input = "16,1,2,0,4,2,7,1,2,14\n".to_string();
        let expected = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let actual = parse_input(input);

        assert_eq!(actual, expected);
    }
}
