use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Population {
    fishes: [u128; 9],
}

impl Population {
    /// Create a new fish population.
    fn new(fishes: [u128; 9]) -> Population {
        Population { fishes }
    }

    /// Create a new fish population from a list of fish.
    fn from_fish_list(fish_list: Vec<usize>) -> Population {
        let mut population = Population::new([0u128; 9]);

        for fish_time in fish_list {
            population.fishes[fish_time] += 1;
        }

        population
    }

    /// Create a new fish population from an input list of fish times.
    fn from_input(input: String) -> Population {
        let fish_list = input
            .trim()
            .split(",")
            .into_iter()
            .map(|fish_time_str| fish_time_str.parse::<usize>())
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect();

        Population::from_fish_list(fish_list)
    }

    /// Simulate the next day.
    fn next_day(&mut self) {
        let fishes_giving_birth = self.fishes[0];

        // Pass one day on the timer of each fish
        for i in 1..9 {
            self.fishes[i - 1] = self.fishes[i];
        }

        // Give birth to the new fishes
        self.fishes[8] = fishes_giving_birth;
        // Reset timer of the fishes that gave birth
        self.fishes[6] += fishes_giving_birth;
    }

    /// Simulate the population for the given number of days.
    ///
    /// Returns the fish population at the end.
    fn simulate_days(&mut self, days: usize) -> u128 {
        for _ in 0..days {
            self.next_day();
        }

        self.fish_count()
    }

    /// Count the number of fishes.
    fn fish_count(&self) -> u128 {
        self.fishes.into_iter().sum()
    }
}

fn main() {
    let filename = "./input/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut population = Population::from_input(input);

    population.simulate_days(80);
    let count_80_days = population.fish_count();

    println!("There are {} fish after 80 days!", count_80_days);

    population.simulate_days(256 - 80);
    let count_256_days = population.fish_count();

    println!("There are {} fish after 256 days!", count_256_days);
}

#[cfg(test)]
mod tests {
    use crate::Population;
    #[test]
    fn should_create_from_fish_list() {
        // Population 3,4,3,1,2
        let fish_list = vec![3, 4, 3, 1, 2];
        let expected = Population::new([0, 1, 1, 2, 1, 0, 0, 0, 0]);
        let actual = Population::from_fish_list(fish_list);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_create_from_input() {
        let input = "3,4,3,1,2".to_string();
        let expected = Population::new([0, 1, 1, 2, 1, 0, 0, 0, 0]);
        let actual = Population::from_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_simulate_fish_population() {
        let mut population = Population::from_input("3,4,3,1,2\n".into());
        let population_18_days = population.simulate_days(18);
        let population_80_days = population.simulate_days(80 - 18);

        assert_eq!(population_18_days, 26);
        assert_eq!(population_80_days, 5934);
    }
}
