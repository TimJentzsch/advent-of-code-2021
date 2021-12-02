use std::fs;

fn main() {
    // Read the input file
    let filename = "./input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("{}", contents);
}
