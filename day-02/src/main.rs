use std::fs;

/// The current position of the submarine.
struct SubmarinePosition {
    depth: usize,
    horizontal_position: usize,
}

/// A command to control the submarine.
enum SubmarineCommand {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn main() {
    // Read the input file
    let filename = "./input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("{}", contents);
}
