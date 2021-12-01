use std::fs;

fn main() {
    println!("Hello, world!");

    // Read the input file
    let filename = "./input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let depth_increases = count_depth_increases(contents);

    println!("The depth increased {} number of times.", depth_increases);
}

fn count_depth_increases(contents: String) -> u32 {
    // Parse the depth values
    let depths: Vec<u32> = contents
        .split('\n')
        .into_iter()
        // Convert the line to an integer
        .map(|line| str::parse::<u32>(line))
        // Filter out invalid lines
        .filter(|parse_result| match parse_result {
            Ok(_) => true,
            Err(_) => false,
        })
        // Unwrap the result
        .map(|parse_result| parse_result.unwrap())
        .collect();

    let mut depth_increases = 0;

    for (index, depth) in depths.iter().enumerate() {
        if let Some(next_depth) = depths.get(index + 1) {
            if next_depth > depth {
                depth_increases += 1;
            }
        }
    }

    depth_increases
}
