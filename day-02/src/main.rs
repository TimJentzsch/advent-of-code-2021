use std::fs;

#[derive(Debug, PartialEq)]
struct ParseCommandError;

/// The current position of the submarine.
struct SubmarinePosition {
    depth: usize,
    horizontal_position: usize,
}

impl SubmarinePosition {
    /// Executes the given command.
    fn execute_cmd(&mut self, cmd: SubmarineCommand) {
        match cmd {
            SubmarineCommand::Forward(value) => self.horizontal_position += value,
            SubmarineCommand::Down(value) => self.depth += value,
            SubmarineCommand::Up(value) => self.depth -= value,
        }
    }

    /// Execute a list of commands.
    fn execute_cmd_list(&mut self, cmd_list: Vec<SubmarineCommand>) {
        for cmd in cmd_list {
            self.execute_cmd(cmd);
        }
    }
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

/// Parse the integer value of a command
fn parse_command_value(token: Option<&str>) -> Result<usize, ParseCommandError> {
    if let Some(token_str) = token {
        match token_str.parse::<usize>() {
            Ok(value) => Ok(value),
            Err(_) => Err(ParseCommandError),
        }
    } else {
        Err(ParseCommandError)
    }
}

/// Parse a single command.
fn parse_command(line: String) -> Result<SubmarineCommand, ParseCommandError> {
    let mut tokens = line.split_whitespace();

    match tokens.next() {
        Some("forward") => {
            if let Ok(value) = parse_command_value(tokens.next()) {
                Ok(SubmarineCommand::Forward(value))
            } else {
                Err(ParseCommandError)
            }
        }
        Some("down") => {
            if let Ok(value) = parse_command_value(tokens.next()) {
                Ok(SubmarineCommand::Down(value))
            } else {
                Err(ParseCommandError)
            }
        }
        Some("up") => {
            if let Ok(value) = parse_command_value(tokens.next()) {
                Ok(SubmarineCommand::Up(value))
            } else {
                Err(ParseCommandError)
            }
        }
        _ => Err(ParseCommandError),
    }
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
