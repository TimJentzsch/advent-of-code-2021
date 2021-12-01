use std::{fs, iter::repeat};

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
fn parse_depths(contents: String) -> Vec<usize> {
    contents
        .split('\n')
        .into_iter()
        // Convert the line to an integer
        .map(|line| str::parse::<usize>(line))
        // Filter out invalid lines
        .filter(|parse_result| match parse_result {
            Ok(_) => true,
            Err(_) => false,
        })
        // Unwrap the result
        .map(|parse_result| parse_result.unwrap())
        .collect()
}

fn create_sliding_windows(depths: Vec<usize>, size: usize) -> Vec<usize> {
    let window_count = depths.len() - (size - 1);
    // Create the sliding windows, initialized with zeroes
    // See https://stackoverflow.com/a/28208182
    let mut sliding_windows: Vec<usize> = repeat(0).take(window_count).collect();

    // For each depth...
    for (i, depth) in depths.iter().enumerate() {
        // Add it to the number of previous windows according to their size
        for j in 0..size {
            // Make sure the indexes are valid
            if i >= j {
                let index = i - j;
                if index < sliding_windows.len() {
                    sliding_windows[index] += depth;
                }
            }
        }
    }

    sliding_windows
}

/// Count the number of depth increases.
fn count_depth_increases(depths: Vec<usize>) -> usize {
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
fn should_create_sliding_windows() {
    // Example from the task description
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let expected = vec![607, 618, 618, 617, 647, 716, 769, 792];
    let actual = create_sliding_windows(depths, 3);

    assert_eq!(actual, expected);
}

#[test]
fn should_count_depth_increases() {
    // Example from the task description
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let actual = count_depth_increases(depths);

    assert_eq!(actual, 7);
}
