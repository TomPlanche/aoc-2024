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

    ///
    /// # from_str
    /// Parse the input string to a Data struct
    ///
    /// ## Arguments
    /// * `s` - The input string
    ///
    /// ## Returns
    /// * `Result<Self, Self::Err>` - The Data struct
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

///
/// # is_always_increasing
/// Check if the levels are always increasing
///
/// ## Arguments
/// * `levels` - The levels to check
///
/// ## Returns
/// * `bool` - True if the levels are always increasing
fn is_always_increasing(levels: &Vec<i32>) -> bool {
    for i in 1..levels.len() {
        if levels[i] < levels[i - 1] {
            return false;
        }
    }

    true
}

impl Data {
    /// # is_report_safe
    /// Check if the report is safe
    /// A report is safe if the difference between each level is between 1 and 3 and the levels are always increasing or decreasing.
    ///
    /// ## Arguments
    /// * `levels` - The levels to check
    ///
    /// ## Returns
    /// * `bool` - True if the report is safe
    fn is_report_safe(&self, levels: &Vec<i32>) -> bool {
        let mut levels = levels.clone();

        if levels.len() < 2 {
            return true;
        }

        for i in 1..levels.len() {
            let diff = (levels[i] - levels[i - 1]).abs();
            if !(1..3).contains(&diff) {
                return false;
            }
        }

        if !is_always_increasing(&levels) {
            levels.reverse();

            return is_always_increasing(&levels);
        }

        true
    }

    ///
    /// # is_report_safe_with_dampener
    /// Check if the report is safe with a dampener
    /// See if the report is safe by removing one level.
    ///
    /// ## Arguments
    /// * `levels` - The levels to check
    ///
    /// ## Returns
    /// * `bool` - True if the report is safe with a dampener
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

    ///
    /// # count_safe_arrangements
    /// Count the number of safe arrangements
    ///
    /// ## Returns
    /// * `usize` - The number of safe arrangements
    fn count_safe_arrangements(&self) -> usize {
        self.levels
            .iter()
            .filter(|&report| self.is_report_safe(report))
            .count()
    }

    ///
    /// # count_safe_arrangements_with_dampener
    /// Count the number of safe arrangements with a dampener
    ///
    /// ## Returns
    /// * `usize` - The number of safe arrangements with a dampener
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1 2 3\n4 5 6";
        let data = Data::from_str(input).unwrap();
        assert_eq!(data.levels, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_is_always_increasing() {
        assert!(is_always_increasing(&vec![1, 2, 3, 4]));
        assert!(!is_always_increasing(&vec![1, 3, 2, 4]));
        assert!(!is_always_increasing(&vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_is_report_safe() {
        let data = Data { levels: vec![] };

        // Test increasing sequence
        assert!(data.is_report_safe(&vec![1, 2, 3, 4]));

        // Test decreasing sequence
        assert!(data.is_report_safe(&vec![4, 3, 2, 1]));

        // Test invalid differences
        assert!(!data.is_report_safe(&vec![1, 4, 7]));

        // Test single value
        assert!(data.is_report_safe(&vec![1]));

        // Test empty sequence
        assert!(data.is_report_safe(&vec![]));
    }

    #[test]
    fn test_is_report_safe_with_dampener() {
        let data = Data { levels: vec![] };

        // Already safe sequence
        assert!(data.is_report_safe_with_dampener(&vec![1, 2, 3]));

        // Can become safe by removing middle element
        assert!(data.is_report_safe_with_dampener(&vec![1, 5, 3]));

        // Cannot become safe by removing any element
        assert!(!data.is_report_safe_with_dampener(&vec![1, 5, 8]));
    }

    #[test]
    fn test_count_safe_arrangements() {
        let input = "1 2 3\n1 4 2\n1 2 5";
        let data = Data::from_str(input).unwrap();
        assert_eq!(data.count_safe_arrangements(), 1);
    }
}
