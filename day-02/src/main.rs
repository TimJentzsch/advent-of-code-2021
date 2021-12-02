use std::{fs, vec, string::ParseError};

/// The current position of the submarine.
struct SubmarinePosition {
    depth: usize,
    horizontal_position: usize,
}

/// A command to control the submarine.
#[derive(Debug, PartialEq)]
enum SubmarineCommand {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn main() {
    // Read the input file
    let filename = "./input/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("{}", input);
}

/// Parse a single command.
fn parse_command(_line: String) -> Result<SubmarineCommand, ParseError> {
    Ok(SubmarineCommand::Down(0))
}

/// Parse the given input to commands.
fn parse_command_list(input: String) -> Vec<SubmarineCommand> {
    input
        .split('\n')
        .into_iter()
        // Parse the line to a command
        .map(|line| parse_command(line.to_string()))
        // Filter out invalid lines
        .filter(|parse_result| match parse_result {
            Ok(_) => true,
            Err(_) => false,
        })
        // Unwrap the result
        .map(|parse_result| parse_result.unwrap())
        .collect()
}

#[test]
fn should_parse_forward_command() {
    let input = "forward 5".to_string();
    let expected = Ok(SubmarineCommand::Forward(5));
    let actual = parse_command(input);

    assert_eq!(actual, expected);
}

#[test]
fn should_parse_down_command() {
    let input = "down 5".to_string();
    let expected = Ok(SubmarineCommand::Down(5));
    let actual = parse_command(input);

    assert_eq!(actual, expected);
}

#[test]
fn should_parse_up_command() {
    let input = "up 3".to_string();
    let expected = Ok(SubmarineCommand::Up(3));
    let actual = parse_command(input);

    assert_eq!(actual, expected);
}

#[test]
fn should_parse_command_list() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".to_string();
    let expected = vec![
        SubmarineCommand::Forward(5),
        SubmarineCommand::Down(5),
        SubmarineCommand::Forward(8),
        SubmarineCommand::Up(3),
        SubmarineCommand::Down(8),
        SubmarineCommand::Forward(2),
    ];
    let actual = parse_command_list(input);

    assert_eq!(actual, expected);
}
