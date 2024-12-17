///
/// # day_17.rs
/// Code for the day 17 of the Advent of Code challenge year 2024
///
/// ## Implementation Details
/// This implementation simulates a 3-bit computer with the following features:
///
/// - Three registers (A, B, C) that can hold integers
/// - Eight 3-bit instructions (0-7)
/// - Two types of operands (literal and combo)
/// - Program execution with instruction pointer management
///
/// ## Instruction Set
/// - 0 (adv): Division of register A by 2^operand
/// - 1 (bxl): XOR of register B with literal operand
/// - 2 (bst): Store operand mod 8 in register B
/// - 3 (jnz): Jump if register A is non-zero
/// - 4 (bxc): XOR of registers B and C
/// - 5 (out): Output operand mod 8
/// - 6 (bdv): Division result to register B
/// - 7 (cdv): Division result to register C
// Imports  ==============================================================================  Imports
use regex::Regex;
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_17.txt");

/// Represents the 3-bit computer with registers and program execution state
#[derive(Debug)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<i64>,
    instruction_pointer: usize,
    output: Vec<i64>,
}

impl Computer {
    ///
    /// # `new`
    /// Creates a new Computer instance with initial register values and program
    ///
    /// ## Arguments
    /// * `register_a` - Initial value for register A
    /// * `register_b` - Initial value for register B
    /// * `register_c` - Initial value for register C
    /// * `program` - Vector of instructions to execute
    fn new(register_a: i64, register_b: i64, register_c: i64, program: Vec<i64>) -> Self {
        Computer {
            register_a,
            register_b,
            register_c,
            program,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    ///
    /// # `get_combo_value`
    /// Resolves the value of a combo operand based on its type
    ///
    /// ## Arguments
    /// * `operand` - The combo operand value (0-7)
    ///
    /// ## Returns
    /// The resolved value based on the combo operand rules
    ///
    /// ## Panics
    /// Panics if the operand is 7 or invalid
    fn get_combo_value(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Invalid combo operand 7"),
            _ => panic!("Invalid combo operand"),
        }
    }

    ///
    /// # `run`
    /// Executes the program until completion
    ///
    /// Processes instructions sequentially, updating registers and output
    /// as specified by the instruction set. The instruction pointer is
    /// advanced by 2 after each instruction unless modified by a jump.
    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];

            match opcode {
                0 => {
                    // adv
                    let divisor = 1 << self.get_combo_value(operand); // `1 << n` is equivalent to 2^n
                    self.register_a /= divisor;
                }
                1 => {
                    // bxl (xor literal)
                    self.register_b ^= operand;
                }
                2 => {
                    // bst (combo mod 8)
                    self.register_b = self.get_combo_value(operand) % 8;
                }
                3 => {
                    // jnz
                    if self.register_a != 0 {
                        self.instruction_pointer = operand as usize;
                        continue;
                    }
                }
                4 => {
                    // bxc (bitwise XOR)
                    self.register_b ^= self.register_c;
                }
                5 => {
                    // out (calculate and output)
                    self.output.push(self.get_combo_value(operand) % 8);
                }
                6 => {
                    // bdv
                    let divisor = 1 << self.get_combo_value(operand);
                    self.register_b = self.register_a / divisor;
                }
                7 => {
                    // cdv
                    let divisor = 1 << self.get_combo_value(operand);
                    self.register_c = self.register_a / divisor;
                }
                _ => panic!("Invalid opcode"),
            }
            self.instruction_pointer += 2;
        }
    }
}

/// Represents the input format for the program including initial register values
#[derive(Debug)]
struct ProgramInput {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<i64>,
}

impl FromStr for ProgramInput {
    type Err = String;

    ///
    /// # `from_str`
    /// Parses the program input from a string
    ///
    /// ## Format
    /// ```text
    /// Register A: <value>
    /// Register B: <value>
    /// Register C: <value>
    ///
    /// Program: <comma-separated values>
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let register_pattern = Regex::new(r"Register ([ABC]): (-?\d+)").unwrap();
        let program_pattern = Regex::new(r"Program: ([-\d,\s]+)").unwrap();

        let mut register_a = None;
        let mut register_b = None;
        let mut register_c = None;

        // Parse registers
        for cap in register_pattern.captures_iter(s) {
            let value = cap[2]
                .parse::<i64>()
                .map_err(|_| "Failed to parse register value")?;

            match &cap[1] {
                "A" => register_a = Some(value),
                "B" => register_b = Some(value),
                "C" => register_c = Some(value),
                _ => unreachable!(),
            }
        }

        // Parse program
        let program = program_pattern
            .captures(s)
            .ok_or("Failed to find program")?[1]
            .split(',')
            .map(|num| num.trim().parse())
            .collect::<Result<Vec<i64>, _>>()
            .map_err(|_| "Failed to parse program numbers")?;

        Ok(ProgramInput {
            register_a: register_a.ok_or("Missing Register A")?,
            register_b: register_b.ok_or("Missing Register B")?,
            register_c: register_c.ok_or("Missing Register C")?,
            program,
        })
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 17 - Part 1");
    let start = std::time::Instant::now();

    let input: ProgramInput = INPUT.parse().unwrap();
    let mut computer = Computer::new(
        input.register_a,
        input.register_b,
        input.register_c,
        input.program,
    );

    computer.run();

    let output = computer
        .output
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let duration = start.elapsed();

    println!("Output: {output}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 17 - Part 2");
    let start = std::time::Instant::now();

    let input: ProgramInput = INPUT.parse().unwrap();
    let program = input.program;

    let mut a = 0;
    // Iterate through positions from end to start
    for pos in (0..program.len()).rev() {
        // Shift left by 3 bits for each position
        a <<= 3;

        // Try values until we find one that outputs the correct sequence
        loop {
            let mut computer =
                Computer::new(a, input.register_b, input.register_c, program.clone());
            computer.run();

            // Check if the output matches the expected sequence
            let expected: Vec<i64> = program[pos..].to_vec();
            if computer.output == expected {
                break;
            }
            a += 1;
        }
    }

    let duration = start.elapsed();
    println!("Result: {a}");
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
    fn test_example_program() {
        let input = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let program_input: ProgramInput = input.parse().unwrap();
        let mut computer = Computer::new(
            program_input.register_a,
            program_input.register_b,
            program_input.register_c,
            program_input.program,
        );

        computer.run();

        let expected = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];
        assert_eq!(computer.output, expected);
    }

    #[test]
    fn test_bst_instruction() {
        let program = vec![2, 6]; // bst instruction with operand 6 (register C)
        let mut computer = Computer::new(0, 0, 9, program);
        computer.run();
        assert_eq!(computer.register_b, 1); // 9 % 8 = 1
    }

    #[test]
    fn test_out_instruction() {
        let program = vec![5, 0, 5, 1, 5, 4]; // multiple out instructions
        let mut computer = Computer::new(10, 0, 0, program);
        computer.run();
        assert_eq!(computer.output, vec![0, 1, 2]);
    }
}
