use std::fs;

fn main() {
    println!("Hello, world!");

    // Read the input file
    let filename = "./input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let depths = parse_depths(contents);
    let depth_increases = count_depth_increases(depths);

    println!("The depth increased {} number of times.", depth_increases);
}

/// Parse the depth values provided in the file.
fn parse_depths(contents: String) -> Vec<u32> {
    contents
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
        .collect()
}

/// Count the number of depth increases.
fn count_depth_increases(depths: Vec<u32>) -> u32 {
    // Parse the depth values
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

#[test]
fn should_parse_depths() {
    // Example from the task description
    let contents = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n".to_string();
    let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let actual = parse_depths(contents);

    assert_eq!(actual, expected);
}

#[test]
fn should_count_depth_increases() {
    // Example from the task description
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let actual = count_depth_increases(depths);

    assert_eq!(actual, 7);
}
