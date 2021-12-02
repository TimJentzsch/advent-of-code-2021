use std::fs;

#[derive(Debug, PartialEq)]
struct ParseCommandError;

/// The current position of the submarine.
#[derive(Debug, PartialEq)]
struct SubmarinePosition {
    depth: usize,
    horizontal_position: usize,
    aim: usize,
}

impl SubmarinePosition {
    /// Create a new submarine position.
    fn new() -> Self {
        SubmarinePosition {
            depth: 0,
            horizontal_position: 0,
            aim: 0,
        }
    }

    /// Executes the given command.
    fn execute_cmd(&mut self, cmd: SubmarineCommand) {
        match cmd {
            SubmarineCommand::Forward(value) => {
                self.horizontal_position += value;
                self.depth += self.aim * value;
            }
            SubmarineCommand::Down(value) => self.aim += value,
            SubmarineCommand::Up(value) => self.aim -= value,
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

    let cmd_list = parse_command_list(input);

    let mut pos = SubmarinePosition::new();

    pos.execute_cmd_list(cmd_list);

    println!(
        "The submarine is at depth {} and horizontal position {} (value {})",
        pos.depth,
        pos.horizontal_position,
        pos.depth * pos.horizontal_position
    );
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

#[test]
fn should_execute_commands() {
    let cmd_list = vec![
        SubmarineCommand::Forward(5),
        SubmarineCommand::Down(5),
        SubmarineCommand::Forward(8),
        SubmarineCommand::Up(3),
        SubmarineCommand::Down(8),
        SubmarineCommand::Forward(2),
    ];
    let mut actual = SubmarinePosition::new();
    actual.execute_cmd_list(cmd_list);
    let expected = SubmarinePosition {
        aim: 10,
        depth: 60,
        horizontal_position: 15,
    };

    assert_eq!(actual, expected);
}
