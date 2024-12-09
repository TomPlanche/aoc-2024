///
/// # day_02.rs
/// Code for the day 02 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_02.txt");
const MAX_LEVEL_DIFF: i32 = 3;

///
/// # ReactorReport
/// Represents a collection of reactor level readings that need to be analyzed for safety
struct ReactorReport {
    readings: Vec<Vec<i32>>,
}

impl FromStr for ReactorReport {
    type Err = ();

    ///
    /// # from_str
    /// Parses multiple lines of space-separated numbers into reactor level readings
    ///
    /// ## Arguments
    /// * `s` - Raw input string containing reactor readings
    ///
    /// ## Returns
    /// * `Result<Self, Self::Err>` - Parsed reactor report or error
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let readings = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();

        Ok(ReactorReport { readings })
    }
}

///
/// # is_monotonic
/// Checks if a sequence of levels is strictly increasing or decreasing
///
/// ## Arguments
/// * `levels` - Vector of reactor levels to check
///
/// ## Returns
/// * `bool` - True if levels are monotonic
fn is_monotonic(levels: &[i32]) -> bool {
    let increasing = levels.windows(2).all(|w| w[1] > w[0]);
    let decreasing = levels.windows(2).all(|w| w[1] < w[0]);

    increasing || decreasing
}

impl ReactorReport {
    ///
    /// # is_reading_safe
    /// Determines if a single reactor reading is safe according to safety rules:
    /// - Levels must be monotonic (all increasing or all decreasing)
    /// - Adjacent levels must differ by 1 to 3 units
    ///
    /// ## Arguments
    /// * `levels` - Vector of reactor levels to check
    ///
    /// ## Returns
    /// * `bool` - True if the reading is safe
    fn is_reading_safe(&self, levels: &[i32]) -> bool {
        if levels.len() < 2 {
            return true;
        }

        // Check if differences are valid (between 1 and 3)
        let valid_differences = levels.windows(2).all(|w| {
            let diff = (w[1] - w[0]).abs();
            diff <= MAX_LEVEL_DIFF // Changed: Only check upper bound
        });

        if !valid_differences {
            return false;
        }

        // Then check if sequence is monotonic
        is_monotonic(levels)
    }

    ///
    /// # is_reading_safe_with_dampener
    /// Checks if a reading can be made safe by removing one level (Problem Dampener)
    ///
    /// ## Arguments
    /// * `levels` - Vector of reactor levels to check
    ///
    /// ## Returns
    /// * `bool` - True if the reading can be made safe
    fn is_reading_safe_with_dampener(&self, levels: &[i32]) -> bool {
        if levels.len() < 2 {
            return true;
        }

        // Check if already safe
        if self.is_reading_safe(levels) {
            return true;
        }

        // Try removing each element and check if resulting sequence is safe
        (0..levels.len()).any(|i| {
            let mut modified = levels.to_vec();
            modified.remove(i);
            self.is_reading_safe(&modified)
        })
    }

    /// Count safe readings without Problem Dampener
    fn count_safe_readings(&self) -> usize {
        self.readings
            .iter()
            .filter(|reading| self.is_reading_safe(reading))
            .count()
    }

    /// Count safe readings with Problem Dampener
    fn count_safe_readings_with_dampener(&self) -> usize {
        self.readings
            .iter()
            .filter(|reading| self.is_reading_safe_with_dampener(reading))
            .count()
    }
}
// Functions  =========================================================================== Functions

pub fn response_part_1() {
    println!("Day 02 - Part 1");

    let start = std::time::Instant::now();

    let count = ReactorReport::from_str(INPUT)
        .unwrap()
        .count_safe_readings();

    let duration = start.elapsed();

    println!("Count: {}", count);
    println!("Duration: {duration:?}\n");
}

pub fn response_part_2() {
    println!("Day 02 - Part 2");

    let start = std::time::Instant::now();

    let count = ReactorReport::from_str(INPUT)
        .unwrap()
        .count_safe_readings_with_dampener();

    let duration = start.elapsed();

    println!("Count: {}", count);
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

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse_input() {
        let input = "1 2 3\n4 5 6";
        let data = ReactorReport::from_str(input).unwrap();

        assert_eq!(data.readings, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_is_always_increasing() {
        assert!(is_monotonic(&[1, 2, 3, 4])); // increasing
        assert!(!is_monotonic(&[1, 3, 2, 4])); // not monotonic
        assert!(is_monotonic(&[4, 3, 2, 1])); // decreasing - should be true!
    }

    #[test]
    fn test_is_reading_safe() {
        let report = ReactorReport::from_str("1 2 3").unwrap();

        // Basic increasing/decreasing sequences
        assert!(report.is_reading_safe(&[1, 2, 3])); // Increasing, valid differences
        assert!(report.is_reading_safe(&[3, 2, 1])); // Decreasing, valid differences

        // Invalid differences
        assert!(!report.is_reading_safe(&[1, 5, 6])); // Too large difference (4)

        // Non-monotonic sequences
        assert!(!report.is_reading_safe(&[1, 3, 2])); // Up then down
        assert!(!report.is_reading_safe(&[2, 1, 3])); // Down then up

        // Edge cases
        assert!(report.is_reading_safe(&[1])); // Single element
        assert!(report.is_reading_safe(&[])); // Empty sequence
        assert!(report.is_reading_safe(&[1, 2])); // Two elements, valid difference
    }

    #[test]
    fn test_is_reading_safe_with_dampener() {
        let report = ReactorReport::from_str("1 2 3").unwrap();

        // Already safe sequences
        assert!(report.is_reading_safe_with_dampener(&[1, 2, 3])); // Already safe
        assert!(report.is_reading_safe_with_dampener(&[3, 2, 1])); // Already safe

        // Can be made safe by removing one element
        assert!(report.is_reading_safe_with_dampener(&[1, 3, 2])); // Remove 3 to get [1, 2]
        assert!(report.is_reading_safe_with_dampener(&[1, 4, 2])); // Remove 4 to get [1, 2]
        assert!(report.is_reading_safe_with_dampener(&[5, 1, 2, 3])); // Remove 5 to get [1, 2, 3]

        // Cannot be made safe by removing one element
        assert!(!report.is_reading_safe_with_dampener(&[1, 5, 2, 6])); // Multiple issues
        assert!(!report.is_reading_safe_with_dampener(&[1, 5, 2, 4, 3])); // Too many direction changes

        // Edge cases
        assert!(report.is_reading_safe_with_dampener(&[1])); // Single element
        assert!(report.is_reading_safe_with_dampener(&[])); // Empty sequence
        assert!(report.is_reading_safe_with_dampener(&[1, 2])); // Two elements
    }

    #[test]
    fn test_count_safe_arrangements() {
        let input = "1 2 3\n1 4 2\n1 2 5";
        let data = ReactorReport::from_str(input).unwrap();

        assert_eq!(data.count_safe_readings(), 2); // Only [1,2,3] is strictly monotonic with valid differences
    }

    #[test]
    fn test_count_safe_arrangements_from_example() {
        let data = ReactorReport::from_str(INPUT).unwrap();

        assert_eq!(data.count_safe_readings(), 2);
    }

    #[test]
    fn test_count_safe_arrangements_with_dampener() {
        let input = "1 2 3\n1 4 2\n1 2 5";
        let data = ReactorReport::from_str(input).unwrap();

        assert_eq!(data.count_safe_readings_with_dampener(), 3); // Adjusted based on dampener rules
    }
}
