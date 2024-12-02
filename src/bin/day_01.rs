///
/// # day_01.rs
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

    println!("Sum: {}", sum);
}

pub fn response_part_2() {
    println!("Day 01 - Part 2");

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

    println!("Sum: {}", sum);
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
