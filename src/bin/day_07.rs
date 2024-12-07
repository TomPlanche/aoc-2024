///
/// # `day_07.rs`
/// Code for the day 07 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_07.txt");

#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

impl FromStr for Equation {
    type Err = ();

    ///
    /// # `from_str`
    /// Parse a string into an Equation
    /// A line is formatted as:
    /// 21037: 9 7 18 13
    ///
    /// number: number number number number
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equation_regex =
            regex::Regex::new(r"^(?P<result>\d+):(?P<numbers>(?:\s\d+)+)$").unwrap();
        let captures = equation_regex.captures(s).unwrap();

        let result = captures.name("result").unwrap().as_str().parse().unwrap();
        let numbers = captures
            .name("numbers")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Equation { result, numbers })
    }
}

impl Equation {
    ///
    /// # `is_valid`
    /// Check if the equation can be solved using + and * operators
    ///
    /// ## Arguments
    /// * `use_concatenation` - True if the concatenation operator can be used
    ///
    /// ## Returns
    ///
    /// * `bool` - True if the equation can be solved, false otherwise
    fn is_valid(&self, use_concatenation: bool) -> bool {
        self.try_all_combinations(0, self.numbers[0], use_concatenation)
    }

    ///
    /// # `concatenate`
    /// Concatenate two numbers
    ///
    /// ## Arguments
    ///
    /// * `a` - First number
    /// * `b` - Second number
    ///
    /// ## Returns
    ///
    /// * `i64` - Concatenated number
    fn concatenate(a: i64, b: i64) -> i64 {
        let b_str = b.to_string();
        let concatenated = format!("{}{}", a, b_str);

        concatenated.parse().unwrap()
    }

    ///
    /// # `try_all_combinations`
    /// Recursively try all possible combinations of operators (+, *, ||)
    ///
    /// ## Arguments
    ///
    /// * `index` - Current position in the numbers array
    /// * `current` - Current result of the calculation
    /// * `use_concatenation` - True if the concatenation operator can be used
    ///
    /// ## Returns
    ///
    /// * `bool` - True if a valid combination was found
    fn try_all_combinations(&self, index: usize, current: i64, use_concatenation: bool) -> bool {
        if index == self.numbers.len() - 1 {
            return current == self.result;
        }

        // Try addition
        if self.try_all_combinations(
            index + 1,
            current + self.numbers[index + 1],
            use_concatenation,
        ) {
            return true;
        }

        // Try multiplication
        if self.try_all_combinations(
            index + 1,
            current * self.numbers[index + 1],
            use_concatenation,
        ) {
            return true;
        }

        if use_concatenation {
            // Try concatenation
            self.try_all_combinations(
                index + 1,
                Self::concatenate(current, self.numbers[index + 1]),
                use_concatenation,
            )
        } else {
            false
        }
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 07 - Part 1");
    let start = std::time::Instant::now();

    let truc = INPUT
        .trim()
        .lines()
        .map(|line| line.parse::<Equation>().unwrap())
        .filter(|eq| eq.is_valid(false))
        .map(|eq| eq.result)
        .sum::<i64>();

    let duration = start.elapsed();

    println!("Time elapsed: {duration:?}");
    println!("Duration: {duration:?}");

    println!("Result: {truc}");
}

pub fn response_part_2() {
    println!("Day 07 - Part 2");
    let start = std::time::Instant::now();

    let result = INPUT
        .trim()
        .lines()
        .map(|line| line.parse::<Equation>().unwrap())
        .filter(|eq| eq.is_valid(true))
        .map(|eq| eq.result)
        .sum::<i64>();

    let duration = start.elapsed();

    println!("Result: {result}");
    println!("Time elapsed: {duration:?}");
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    const EQUATION_STRING: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_equation_from_str() {
        let equation: Vec<Equation> = EQUATION_STRING
            .trim()
            .lines()
            .map(|x| x.parse().unwrap())
            .collect();

        assert_eq!(equation[0].result, 190);
        assert_eq!(equation[0].numbers, vec![10, 19]);
    }

    #[test]
    fn test_solve_part_1() {
        let equation: Vec<Equation> = EQUATION_STRING
            .trim()
            .lines()
            .map(|x| x.parse().unwrap())
            .collect();

        let result = equation
            .iter()
            .filter(|eq| eq.is_valid(false))
            .map(|eq| eq.result)
            .sum::<i64>();

        assert_eq!(result, 3749);
    }

    #[test]
    fn test_equation_validity() {
        let equation: Equation = "190: 10 19".parse().unwrap();
        assert!(equation.is_valid(false));

        let equation: Equation = "83: 17 5".parse().unwrap();
        assert!(!equation.is_valid(false));
    }

    #[test]
    fn test_solve_part_2() {
        let equation: Vec<Equation> = EQUATION_STRING
            .trim()
            .lines()
            .map(|x| x.parse().unwrap())
            .collect();

        let result = equation
            .iter()
            .filter(|eq| eq.is_valid(true))
            .map(|eq| eq.result)
            .sum::<i64>();

        assert_eq!(result, 11387);
    }

    #[test]
    fn test_concatenation() {
        let equation: Equation = "156: 15 6".parse().unwrap();
        assert!(equation.is_valid(true));

        let equation: Equation = "7290: 6 8 6 15".parse().unwrap();
        assert!(equation.is_valid(true));

        let equation: Equation = "192: 17 8 14".parse().unwrap();
        assert!(equation.is_valid(true));
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(Equation::concatenate(15, 6), 156);
        assert_eq!(Equation::concatenate(12, 345), 12345);
    }
}
