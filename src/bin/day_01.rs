///
/// # `day_01.rs`
/// Code for the day 01 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::{collections::HashMap, str::FromStr};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_01.txt");

#[derive(Debug)]
struct Data {
    left_values: Vec<i32>,
    right_values: Vec<i32>,
}

// Since the input is a list of '   ' separated values, we can split the input
// and parse each value to an integer.
/**
Ex:
15244   50562
81245   49036
92897   21393
63271   60643
*/
impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left_values = Vec::new();
        let mut right_values = Vec::new();

        for line in s.lines() {
            let mut values = line.split_whitespace();
            left_values.push(values.next().unwrap().parse().unwrap());
            right_values.push(values.next().unwrap().parse().unwrap());
        }

        Ok(Data {
            left_values,
            right_values,
        })
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 01 - Part 1");

    let start = std::time::Instant::now();

    let data: Data = INPUT.parse().unwrap();
    let mut left_values = data.left_values;
    let mut right_values = data.right_values;

    left_values.sort();
    right_values.sort();

    let sum: i32 = left_values
        .iter()
        .zip(right_values.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    let duration = start.elapsed();

    println!("Sum: {sum}");
    println!("Duration: {duration:?}\n");
}

pub fn response_part_2() {
    println!("Day 01 - Part 2");

    let start = std::time::Instant::now();

    let data: Data = INPUT.parse().unwrap();
    let left_values = data.left_values;
    let right_values = data.right_values;

    let mut right_values_count: HashMap<i32, u32> = std::collections::HashMap::new();
    for value in right_values.iter() {
        *right_values_count.entry(*value).or_insert(0) += 1;
    }

    let sum: u32 = left_values
        .iter()
        .map(|value| *value as u32 * right_values_count.get(value).unwrap_or(&0))
        .sum();

    let duration = start.elapsed();

    println!("Sum: {sum}");
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
    fn test_parse_basic_input() {
        let input = "15244   50562\n81245   49036";
        let data = input.parse::<Data>().unwrap();

        assert_eq!(data.left_values, vec![15244, 81245]);
        assert_eq!(data.right_values, vec![50562, 49036]);
    }

    #[test]
    fn test_parse_with_variable_spacing() {
        let input = "123     456\n789   101112";
        let data = input.parse::<Data>().unwrap();

        assert_eq!(data.left_values, vec![123, 789]);
        assert_eq!(data.right_values, vec![456, 101112]);
    }

    #[test]
    fn test_part1_calculation() {
        let input = "100   150\n200   175\n300   250";
        let data = input.parse::<Data>().unwrap();
        let mut left_values = data.left_values;
        let mut right_values = data.right_values;

        left_values.sort();
        right_values.sort();

        let sum: i32 = left_values
            .iter()
            .zip(right_values.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        assert_eq!(sum, 125); // |100-150| + |200-175| + |300-250| = 50 + 25 + 50 = 125
    }

    #[test]
    fn test_part2_calculation() {
        let input = "100   100\n200   200\n100   300\n200   100";
        let data = input.parse::<Data>().unwrap();

        let mut right_values_count: HashMap<i32, u32> = HashMap::new();
        for value in data.right_values.iter() {
            *right_values_count.entry(*value).or_insert(0) += 1;
        }

        let sum: u32 = data
            .left_values
            .iter()
            .map(|value| *value as u32 * right_values_count.get(value).unwrap_or(&0))
            .sum();

        assert_eq!(sum, 800); // 100 * 2 + 200 * 1 + 100 * 2 + 200 * 2 = 800
    }

    #[test]
    fn test_no_matching_values_part2() {
        let input = "100   200\n300   400";
        let data = input.parse::<Data>().unwrap();

        let mut right_values_count: HashMap<i32, u32> = HashMap::new();
        for value in data.right_values.iter() {
            *right_values_count.entry(*value).or_insert(0) += 1;
        }

        let sum: u32 = data
            .left_values
            .iter()
            .map(|value| *value as u32 * right_values_count.get(value).unwrap_or(&0))
            .sum();

        assert_eq!(sum, 0); // No matching values between left and right
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let data = input.parse::<Data>().unwrap();

        assert!(data.left_values.is_empty());
        assert!(data.right_values.is_empty());
    }
}
