///
/// # `day_03.rs`
/// Code for the day 03 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_03.txt");

///
/// # `Instruction`
/// Represents different types of instructions that can be found in the corrupted memory
///
#[derive(Debug)]
enum Instruction {
    Multiply(usize, usize),
    Do,
    Dont,
}

///
/// # `Program`
/// Represents the parsed program with its sequence of instructions
///
#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = ();

    ///
    /// # `from_str`
    /// Parses the input string to extract valid instructions
    ///
    /// ## Arguments
    /// * `s` - The input string containing corrupted memory
    ///
    /// ## Returns
    /// * `Result<Program, ()>` - The parsed program or an error
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();

        // Compile regexes for different instructions
        let mul_regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let do_regex = regex::Regex::new(r"do\(\)").unwrap();
        let dont_regex = regex::Regex::new(r"don't\(\)").unwrap();

        // Track position to process instructions in order
        let mut pos = 0;
        while pos < s.len() {
            if let Some(mat) = mul_regex.find_at(s, pos) {
                if mat.start() == pos {
                    let caps = mul_regex.captures(&s[pos..mat.end()]).unwrap();
                    let a = caps[1].parse().unwrap();
                    let b = caps[2].parse().unwrap();
                    instructions.push(Instruction::Multiply(a, b));
                    pos = mat.end();
                    continue;
                }
            }
            if let Some(mat) = do_regex.find_at(s, pos) {
                if mat.start() == pos {
                    instructions.push(Instruction::Do);
                    pos = mat.end();
                    continue;
                }
            }
            if let Some(mat) = dont_regex.find_at(s, pos) {
                if mat.start() == pos {
                    instructions.push(Instruction::Dont);
                    pos = mat.end();
                    continue;
                }
            }
            pos += 1;
        }

        Ok(Program { instructions })
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 03 - Part 1");

    let start = std::time::Instant::now();

    let sum: usize = INPUT
        .parse::<Program>()
        .unwrap()
        .instructions
        .iter()
        .filter_map(|inst| match inst {
            Instruction::Multiply(a, b) => Some(a * b),
            _ => None,
        })
        .sum();

    let duration = start.elapsed();

    println!("The sum of all multiplications is: {sum}");
    println!("Duration: {duration:?}\n");
}

pub fn response_part_2() {
    println!("Day 03 - Part 2");

    let start = std::time::Instant::now();

    let program = INPUT.parse::<Program>().unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for inst in program.instructions {
        match inst {
            Instruction::Multiply(a, b) if enabled => sum += a * b,
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            _ => {}
        }
    }

    let duration = start.elapsed();

    println!("The sum of all multiplications is: {sum}");
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
    fn test_parse_basic_multiplication() {
        let input = "mul(2,3)";
        let program = input.parse::<Program>().unwrap();
        assert_eq!(program.instructions.len(), 1);
        match &program.instructions[0] {
            Instruction::Multiply(a, b) => {
                assert_eq!(*a, 2);
                assert_eq!(*b, 3);
            }
            _ => panic!("Expected Multiply instruction"),
        }
    }

    #[test]
    fn test_parse_do_dont_instructions() {
        let input = "do()don't()";
        let program = input.parse::<Program>().unwrap();
        assert_eq!(program.instructions.len(), 2);
        match &program.instructions[0] {
            Instruction::Do => (),
            _ => panic!("Expected Do instruction"),
        }
        match &program.instructions[1] {
            Instruction::Dont => (),
            _ => panic!("Expected Dont instruction"),
        }
    }

    #[test]
    fn test_mixed_instructions() {
        let input = "mul(4,5)do()mul(2,3)don't()mul(6,7)";
        let program = input.parse::<Program>().unwrap();
        assert_eq!(program.instructions.len(), 5);
    }

    #[test]
    fn test_ignore_invalid_text() {
        let input = "hello mul(1,2) world do() test";
        let program = input.parse::<Program>().unwrap();
        assert_eq!(program.instructions.len(), 2);
    }

    #[test]
    fn test_part1_calculation() {
        let input = "mul(2,3)what()mul(4,5)mul(3,3)";
        let program = input.parse::<Program>().unwrap();
        let sum: usize = program
            .instructions
            .iter()
            .filter_map(|inst| match inst {
                Instruction::Multiply(a, b) => Some(a * b),
                _ => None,
            })
            .sum();
        assert_eq!(sum, 6 + 20 + 9);
    }

    #[test]
    fn test_part2_calculation() {
        let input = "mul(2,3)don't()mul(4,5)do()mul(3,3)";
        let program = input.parse::<Program>().unwrap();
        let mut enabled = true;
        let mut sum = 0;

        for inst in program.instructions {
            match inst {
                Instruction::Multiply(a, b) if enabled => sum += a * b,
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                _ => {}
            }
        }
        assert_eq!(sum, 6 + 9); // 4*5 is skipped due to don't()
    }
}
