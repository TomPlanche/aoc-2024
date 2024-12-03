///
/// # day_03.rs
/// Code for the day 03 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_03.txt");

///
/// # Instruction
/// Represents different types of instructions that can be found in the corrupted memory
///
#[derive(Debug)]
enum Instruction {
    Multiply(usize, usize),
    Do,
    Dont,
}

///
/// # Program
/// Represents the parsed program with its sequence of instructions
///
#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = ();

    ///
    /// # from_str
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

    println!("The sum of all multiplications is: {}", sum);
}

pub fn response_part_2() {
    println!("Day 03 - Part 2");

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

    println!("The sum of all multiplications is: {}", sum);
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
