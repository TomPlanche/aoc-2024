///
/// # day_11.rs
/// Code for the day 11 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

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
        for _ in 0..n {
            let mut new_arrangement = Vec::new();

            for stone in &self.arrangement {
                if *stone == 0 {
                    new_arrangement.push(1);
                } else if stone.to_string().len() % 2 == 0 {
                    let stone_str = stone.to_string();
                    let half = stone_str.len() / 2;
                    let left = stone_str[..half].parse().unwrap();
                    let right = stone_str[half..].parse().unwrap();
                    new_arrangement.push(left);
                    new_arrangement.push(right);
                } else {
                    new_arrangement.push(stone * 2024);
                }
            }

            self.arrangement = new_arrangement;
        }
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 11 - Part 1");
    let start = std::time::Instant::now();

    let mut stones: Stones = INPUT.parse().unwrap();
    stones.simulate_blinking(25);

    let len = stones.arrangement.len();

    let duration = start.elapsed();

    println!("The number of stones is: {len}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 11 - Part 2");
    let start = std::time::Instant::now();

    let duration = start.elapsed();

    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    //response_part_2();
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
