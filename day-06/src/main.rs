struct Population {
    fishes: [u128; 9],
}

impl Population {
    /// Create a new fish population.
    fn new(fishes: [u128; 9]) -> Population {
        Population { fishes }
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
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Population;

    #[test]
    fn should_simulate_fish_population() {
        // Population 3,4,3,1,2
        let mut population = Population::new([0, 1, 1, 2, 1, 0, 0, 0, 0]);
        let population_18_days = population.simulate_days(18);
        let population_80_days = population.simulate_days(80 - 18);

        assert_eq!(population_18_days, 26);
        assert_eq!(population_80_days, 5934);
    }
}
