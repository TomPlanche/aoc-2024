///
/// # day_02.rs
/// Code for the day 02 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_02.txt");

struct Data {
    levels: Vec<Vec<i32>>,
}

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut levels = Vec::new();

        for line in s.lines() {
            let values = line.split_whitespace();
            let mut level = Vec::new();
            for value in values {
                level.push(value.parse().unwrap());
            }
            levels.push(level);
        }

        Ok(Data { levels })
    }
}

fn is_always_increasing(levels: &Vec<i32>) -> bool {
    for i in 1..levels.len() {
        if levels[i] < levels[i - 1] {
            return false;
        }
    }

    true
}

impl Data {
    fn is_report_safe(&self, levels: &Vec<i32>) -> bool {
        let mut levels = levels.clone();

        if levels.len() < 2 {
            return true;
        }

        for i in 1..levels.len() {
            let diff = (levels[i] - levels[i - 1]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
        }

        if !is_always_increasing(&levels) {
            levels.reverse();

            return is_always_increasing(&levels);
        }

        true
    }

    fn is_report_safe_with_dampener(&self, levels: &Vec<i32>) -> bool {
        if self.is_report_safe(levels) {
            return true;
        }

        for i in 0..levels.len() {
            let mut modified_levels = levels.clone();
            modified_levels.remove(i);
            if self.is_report_safe(&modified_levels) {
                return true;
            }
        }

        false
    }

    fn count_safe_arrangements(&self) -> usize {
        self.levels
            .iter()
            .filter(|&report| self.is_report_safe(report))
            .count()
    }

    fn count_safe_arrangements_with_dampener(&self) -> usize {
        self.levels
            .iter()
            .filter(|&report| self.is_report_safe_with_dampener(report))
            .count()
    }
}
// Functions  =========================================================================== Functions

pub fn response_part_1() {
    println!("Day 02 - Part 1");

    let count = Data::from_str(INPUT).unwrap().count_safe_arrangements();

    println!("Count: {}", count);
}

pub fn response_part_2() {
    println!("Day 02 - Part 2");

    let count = Data::from_str(INPUT)
        .unwrap()
        .count_safe_arrangements_with_dampener();

    println!("Count: {}", count);
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
