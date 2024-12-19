///
/// # day_19.rs
/// Code for the day 19 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::{collections::HashMap, str::FromStr};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_19.txt");

struct TowelGenerator {
    available_towels: Vec<String>,
    desired_designs: Vec<String>,
}

impl FromStr for TowelGenerator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split("\n\n");

        let available_towels = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        let desired_designs = parts
            .next()
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();

        Ok(TowelGenerator {
            available_towels,
            desired_designs,
        })
    }
}

impl TowelGenerator {
    ///
    /// # `is_design_possible`
    /// Check if a given towel design is possible with the available towels
    ///
    /// ## Arguments
    /// * `design` - The design to check
    ///
    /// ## Returns
    /// * `bool` - Whether the design is possible
    fn is_design_possible(&self, design: &str) -> bool {
        self.can_make_pattern(design, 0)
    }

    ///
    /// # `can_make_pattern`
    /// Check if a given pattern can be made with the available towels starting at a given position.
    ///
    /// ## Algorithm
    /// Recursively try to match the pattern starting at the current position with each available towel pattern.
    /// If a pattern fits, recursively try to match the rest of the pattern.
    ///
    /// ## Arguments
    /// * `remaining` - The remaining pattern to match
    /// * `start_pos` - The position to start matching from
    ///
    /// ## Returns
    /// * `bool` - Whether the pattern can be made
    fn can_make_pattern(&self, remaining: &str, start_pos: usize) -> bool {
        // Base case: if we've used up all characters, the pattern is possible
        if start_pos >= remaining.len() {
            return true;
        }

        // Try each available towel pattern at the current position
        for pattern in &self.available_towels {
            if remaining[start_pos..].starts_with(pattern) {
                // If this pattern fits at the current position, recursively try to match the rest
                if self.can_make_pattern(remaining, start_pos + pattern.len()) {
                    return true;
                }
            }
        }

        false
    }

    ///
    /// # `count_possible_designs`
    /// Count the number of possible designs from the desired designs list.
    ///
    /// ## Returns
    /// * `usize` - The number of possible designs
    fn count_possible_designs(&self) -> usize {
        self.desired_designs
            .iter()
            .filter(|design| self.is_design_possible(design))
            .count()
    }

    ///
    /// # `count_arrangements`
    /// Count the number of possible arrangements for a given design.
    ///
    /// ## Arguments
    /// * `design` - The design to count arrangements for
    ///
    /// ## Returns
    /// * `usize` - The number of possible arrangements
    fn count_arrangements(&self, design: &str) -> usize {
        let mut already_computed = HashMap::new();
        let max_pattern_len = self.available_towels.iter().map(|v| v.len()).max().unwrap();

        self.find_arrangements(design, &mut already_computed, max_pattern_len)
    }

    ///
    /// # `find_arrangements`
    /// Find the number of possible arrangements for a given pattern.
    ///
    /// ## Algorithm
    /// Simple recursive algorithm to find all possible arrangements of a pattern with a memoization hashmap.
    ///
    /// ## Arguments
    /// * `pattern` - The pattern to find arrangements for
    /// * `memo` - A memoization hashmap to store already computed values
    /// * `max_len` - The maximum length of a pattern
    ///
    /// ## Returns
    /// * `usize` - The number of possible arrangements
    fn find_arrangements(
        &self,
        pattern: &str,
        memo: &mut HashMap<String, usize>,
        max_len: usize,
    ) -> usize {
        let mut combinations = 0;
        if memo.contains_key(pattern) {
            return *memo.get(pattern).unwrap();
        }
        if pattern.is_empty() {
            return 1;
        }

        for i in 1..=max_len.min(pattern.len()) {
            if self.available_towels.contains(&&pattern[..i].into()) {
                let subcount = self.find_arrangements(&pattern[i..], memo, max_len);
                combinations += subcount;
            }
        }
        memo.insert(pattern.into(), combinations);
        combinations
    }

    ///
    /// `sum_all_arrangements`
    /// Sum the number of possible arrangements for all possible designs.
    ///
    /// ## Returns
    /// * `usize` - The sum of possible arrangements
    fn sum_all_arrangements(&self) -> usize {
        let desired_designs = self
            .desired_designs
            .iter()
            .filter(|design| self.is_design_possible(design));

        desired_designs
            .map(|design| self.count_arrangements(design))
            .sum()
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 19 - Part 1");
    let start = std::time::Instant::now();

    let generator: TowelGenerator = INPUT.parse().unwrap();
    let count = generator.count_possible_designs();

    let duration = start.elapsed();

    println!("Count: {count}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 19 - Part 2");
    let start = std::time::Instant::now();

    let generator: TowelGenerator = INPUT.parse().unwrap();
    let count = generator.sum_all_arrangements();

    let duration = start.elapsed();

    println!("Count: {count}");
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

    const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_from_str() {
        let generator: TowelGenerator = EXAMPLE.parse().unwrap();

        assert_eq!(
            generator.available_towels,
            vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
        );
        assert_eq!(
            generator.desired_designs,
            vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"]
        );
    }

    #[test]
    fn test_is_design_possible() {
        let generator: TowelGenerator = EXAMPLE.parse().unwrap();

        assert_eq!(generator.is_design_possible("brwrr"), true);
        assert_eq!(generator.is_design_possible("bggr"), true);
        assert_eq!(generator.is_design_possible("gbbr"), true);
        assert_eq!(generator.is_design_possible("rrbgbr"), true);
        assert_eq!(generator.is_design_possible("ubwu"), false);
        assert_eq!(generator.is_design_possible("bwurrg"), true);
        assert_eq!(generator.is_design_possible("brgr"), true);
        assert_eq!(generator.is_design_possible("bbrgwb"), false);
    }

    #[test]
    fn test_count_possible_designs() {
        let generator: TowelGenerator = EXAMPLE.parse().unwrap();

        assert_eq!(generator.count_possible_designs(), 6);
    }

    #[test]
    fn test_count_arrangements() {
        let generator: TowelGenerator = EXAMPLE.parse().unwrap();

        assert_eq!(generator.count_arrangements("brwrr"), 2); // b,r,wr,r or br,wr,r
        assert_eq!(generator.count_arrangements("bggr"), 1); // b,g,g,r
        assert_eq!(generator.count_arrangements("gbbr"), 4); // g,b,b,r or g,b,br or gb,b,r or gb,br
        assert_eq!(generator.count_arrangements("rrbgbr"), 6); // All 6 combinations
        assert_eq!(generator.count_arrangements("ubwu"), 0); // Impossible
        assert_eq!(generator.count_arrangements("bwurrg"), 1); // bwu,r,r,g
        assert_eq!(generator.count_arrangements("brgr"), 2); // b,r,g,r or br,g,r
        assert_eq!(generator.count_arrangements("bbrgwb"), 0); // Impossible
    }

    #[test]
    fn test_sum_all_arrangements() {
        let generator: TowelGenerator = EXAMPLE.parse().unwrap();
        assert_eq!(generator.sum_all_arrangements(), 16);
    }
}
