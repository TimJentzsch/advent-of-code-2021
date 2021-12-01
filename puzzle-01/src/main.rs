use std::fs;

fn main() {
    println!("Hello, world!");

    // Read the input file
    let filename = "./input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // Parse the depth values
    let depths: Vec<u16> = contents
        .split('\n')
        .into_iter()
        // Convert the line to an integer
        .map(|line| str::parse::<u16>(line))
        // Filter out invalid lines
        .filter(|parse_result| match parse_result {
            Ok(_) => true,
            Err(_) => false,
        })
        // Unwrap the result
        .map(|parse_result| parse_result.unwrap())
        .collect();

    for depth in depths {
        println!("{}", depth);
    }
}
