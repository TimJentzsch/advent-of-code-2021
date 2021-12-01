use std::fs;

fn main() {
    println!("Hello, world!");

    let filename = "./input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("File contents:\n{}", contents);
}
