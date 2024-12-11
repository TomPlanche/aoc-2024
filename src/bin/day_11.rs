///
/// # day_11.rs
/// Code for the day 11 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_11.txt");

#[derive(Debug)]
struct Stones {
    arrangement: Vec<usize>,
}

impl FromStr for Stones {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arrangement = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        Ok(Stones { arrangement })
    }
}

impl Stones {
    ///
    /// # `simulate_blinking`
    /// Simulate behavior of the stones when we blink.
    /// They will be rearranged in the following way:
    /// They simultaneously change according to the first applicable rule in this list:
    /// - If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    /// - If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    /// - If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
    ///
    /// ## Arguments
    /// * `n` - The number of times to simulate the blinking
    fn simulate_blinking(&mut self, n: usize) {
        const MULTIPLIER: usize = 2024;

        // Use a tuple of (value, digit_count) to avoid recounting digits
        let mut current: Vec<(usize, u32)> = self
            .arrangement
            .iter()
            .map(|&x| {
                (
                    x,
                    if x == 0 {
                        1
                    } else {
                        (x as f64).log10() as u32 + 1
                    },
                )
            })
            .collect();

        for i in 0..n {
            println!("Iteration: {i}");
            let mut next = Vec::with_capacity(current.len() * 2);

            for &(value, digits) in &current {
                if value == 0 {
                    next.push((1, 1));
                } else if digits % 2 == 0 {
                    let power = 10_usize.pow(digits / 2);
                    let left = value / power;
                    let right = value % power;
                    // Calculate new digit counts directly
                    let left_digits = if left == 0 {
                        1
                    } else {
                        (left as f64).log10() as u32 + 1
                    };
                    let right_digits = if right == 0 {
                        1
                    } else {
                        (right as f64).log10() as u32 + 1
                    };
                    next.push((left, left_digits));
                    next.push((right, right_digits));
                } else {
                    let new_value = value * MULTIPLIER;
                    let new_digits = (new_value as f64).log10() as u32 + 1;
                    next.push((new_value, new_digits));
                }
            }

            current = next;
        }

        // Convert back to final arrangement
        self.arrangement = current.into_iter().map(|(value, _)| value).collect();
    }

    ///
    /// # `count_evolved_stones`
    /// Efficiently counts the number of stones after n iterations without maintaining actual values
    /// Uses a combination of queue-based processing and recursive calculation with memoization.
    ///
    /// ## Arguments
    /// * `iterations` - The number of times to simulate the blinking
    ///
    /// ## Returns
    /// * `usize` - The number of stones after n iterations
    fn count_evolved_stones(&self, iterations: usize) -> usize {
        let mut memo: HashMap<(usize, usize), usize> = HashMap::new(); // Cache for (stone, iteration) -> count
        let mut queue: VecDeque<_> = self
            .arrangement
            .iter()
            .map(|&stone| (stone, iterations))
            .collect();

        let mut total = 0;

        // Process each initial stone
        while let Some((stone, iters)) = queue.pop_front() {
            total += self.count_evolved_stones_recursive(stone, iters, &mut memo);
        }

        total
    }

    ///
    /// # `count_evolved_stones_recursive`
    /// Recursive helper function that calculates the number of stones that will evolve from
    /// a single stone after n iterations.
    ///
    /// ## Arguments
    /// * `stone` - The value of the stone
    /// * `iterations` - The number of times to simulate the blinking
    /// * `memo` - A cache for (stone, iteration) -> count
    fn count_evolved_stones_recursive(
        &self,
        stone: usize,
        iterations: usize,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        // Base case: no more iterations
        if iterations == 0 {
            return 1;
        }

        // Check if result is already cached
        if let Some(&count) = memo.get(&(stone, iterations)) {
            return count;
        }

        // Calculate result based on transformation rules
        let result = if stone == 0 {
            // Rule 1: 0 becomes 1
            self.count_evolved_stones_recursive(1, iterations - 1, memo)
        } else {
            let digits = stone.to_string();
            let digit_count = digits.len();

            if digit_count % 2 == 0 {
                // Rule 2: Split even-digit numbers
                let mid = digit_count / 2;
                let left = digits[..mid].parse::<usize>().unwrap();
                let right = digits[mid..].parse::<usize>().unwrap();

                self.count_evolved_stones_recursive(left, iterations - 1, memo)
                    + self.count_evolved_stones_recursive(right, iterations - 1, memo)
            } else {
                // Rule 3: Multiply by 2024
                self.count_evolved_stones_recursive(stone * 2024, iterations - 1, memo)
            }
        };

        // Cache and return result
        memo.insert((stone, iterations), result);
        result
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 11 - Part 1");
    let start = std::time::Instant::now();

    let stones: Stones = INPUT.parse().unwrap();
    let len = stones.count_evolved_stones(25);

    let duration = start.elapsed();

    println!("The number of stones is: {len}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 11 - Part 2");
    let start = std::time::Instant::now();

    let stones: Stones = INPUT.parse().unwrap();
    let len = stones.count_evolved_stones(75);

    let duration = start.elapsed();

    println!("The number of stones is: {len}");
    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "1 2 3 4 5 6 7 8 9 10 10014";

        let stones: Stones = input.parse().unwrap();

        assert_eq!(
            stones.arrangement,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10014]
        );
    }

    #[test]
    fn test_simulate_blinking() {
        let mut stones = Stones {
            arrangement: vec![125, 17],
        };

        stones.simulate_blinking(1);
        assert_eq!(stones.arrangement, vec![253000, 1, 7]);

        stones.simulate_blinking(1);
        assert_eq!(stones.arrangement, vec![253, 0, 2024, 14168]);
    }
}
