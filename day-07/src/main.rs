use std::fs;

fn main() {
    let filename = "./input/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let crabs = parse_input(input);
    let (optimal_pos, fuel_cost) = determine_optimal_position(&crabs);

    println!("The optimal position is {optimal_pos} with a cost of {fuel_cost} fuel.");
}

/// Parse the list of crab positions
fn parse_input(input: String) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .into_iter()
        .map(|value_str| value_str.parse::<i32>())
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .collect()
}

fn determine_optimal_position(crabs: &Vec<i32>) -> (i32, i32) {
    let min = crabs.into_iter().min().unwrap();
    let max = crabs.into_iter().max().unwrap();

    let mut optimum = *min;
    let mut fuel_cost = i32::MAX;

    for pos in *min..(*max + 1) {
        let cur_fuel_cost: i32 = crabs
            .into_iter()
            .map(|crab_pos| {
                let distance = (crab_pos - pos).abs();
                // Sum of all integers in that range
                let cost = (distance * (distance + 1)) / 2;
                cost
            })
            .sum();

        if cur_fuel_cost < fuel_cost {
            optimum = pos;
            fuel_cost = cur_fuel_cost;
        }
    }

    (optimum, fuel_cost)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, determine_optimal_position};

    #[test]
    fn should_parse_input() {
        let input = "16,1,2,0,4,2,7,1,2,14\n".to_string();
        let expected = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let actual = parse_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_optimal_position() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let (expected_pos, expected_fuel) = (5, 168);
        let (actual_pos, actual_fuel) = determine_optimal_position(&crabs);

        assert_eq!(actual_pos, expected_pos);
        assert_eq!(actual_fuel, expected_fuel);
    }
}
