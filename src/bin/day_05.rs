///
/// # day_05.rs
/// Code for day 05 of Advent of Code 2024: Print Queue page ordering verification
///
// Imports  ==============================================================================  Imports
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// Types =================================================================================== Types
/// Represents a page ordering rule where page `before` must come before page `after`
#[derive(Debug)]
struct Rule {
    before: i32,
    after: i32,
}

/// Contains the parsed input data: rules and updates to verify
#[derive(Debug)]
struct PrintQueue {
    rules: Vec<Rule>,
    updates: Vec<Vec<i32>>,
}

// Implementation ======================================================================= Implementation
impl FromStr for PrintQueue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("\n\n").collect();
        if parts.len() != 2 {
            return Err("Invalid input format".to_string());
        }

        // Parse rules
        let rules = parts[0]
            .lines()
            .map(|line| {
                let nums: Vec<i32> = line.split('|').map(|n| n.parse().unwrap()).collect();
                Rule {
                    before: nums[0],
                    after: nums[1],
                }
            })
            .collect();

        // Parse updates
        let updates = parts[1]
            .lines()
            .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
            .collect();

        Ok(PrintQueue { rules, updates })
    }
}

impl PrintQueue {
    /// Checks if a single update follows all applicable ordering rules
    fn is_valid_update(&self, update: &[i32]) -> bool {
        let pages: HashSet<_> = update.iter().collect();

        // Build adjacency map for pages in this update
        let mut after_map: HashMap<i32, HashSet<i32>> = HashMap::new();

        // Only consider rules where both pages are in the update
        for rule in &self.rules {
            if pages.contains(&rule.before) && pages.contains(&rule.after) {
                after_map.entry(rule.before).or_default().insert(rule.after);
            }
        }

        // Check if the order satisfies all rules
        for (i, &page) in update.iter().enumerate() {
            if let Some(must_come_after) = after_map.get(&page) {
                let remaining_pages: HashSet<_> = update[i + 1..].iter().copied().collect();
                if !must_come_after.is_subset(&remaining_pages) {
                    return false;
                }
            }
        }

        true
    }

    /// Gets middle page numbers of valid updates
    fn get_middle_pages(&self) -> Vec<i32> {
        self.updates
            .iter()
            .filter(|update| self.is_valid_update(update))
            .map(|update| update[update.len() / 2])
            .collect()
    }
}

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_05.txt");

// Functions  =========================================================================== Functions

pub fn response_part_1() {
    println!("Day 05 - Part 1");
    let queue: PrintQueue = INPUT.parse().unwrap();
    let middle_sum: i32 = queue.get_middle_pages().iter().sum();
    println!("Sum of middle pages from valid updates: {}", middle_sum);
}

pub fn response_part_2() {
    println!("Day 05 - Part 2");
    // Part 2 not yet available
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
        let input = "47|53\n97|13\n\n75,47,61,53,29\n97,61,53,29,13";
        let queue: PrintQueue = input.parse().unwrap();
        assert_eq!(queue.rules.len(), 2);
        assert_eq!(queue.updates.len(), 2);
        assert_eq!(queue.rules[0].before, 47);
        assert_eq!(queue.rules[0].after, 53);
    }

    #[test]
    fn test_valid_update() {
        let input = "47|53\n47|61\n47|29\n\n75,47,61,53,29";
        let queue: PrintQueue = input.parse().unwrap();
        assert!(queue.is_valid_update(&vec![75, 47, 61, 53, 29]));
    }

    #[test]
    fn test_invalid_update() {
        let input = "97|75\n\n75,97,47,61,53";
        let queue: PrintQueue = input.parse().unwrap();
        assert!(!queue.is_valid_update(&vec![75, 97, 47, 61, 53]));
    }

    #[test]
    fn test_middle_pages() {
        let input = "47|53\n97|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13";
        let queue: PrintQueue = input.parse().unwrap();
        let middle_pages = queue.get_middle_pages();
        assert_eq!(middle_pages, vec![61, 53, 29]);
    }
}
