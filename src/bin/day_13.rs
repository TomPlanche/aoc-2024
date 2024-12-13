use std::str::FromStr;

///
/// # day_13.rs
/// Code for the day 13 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use aoc_2024::Point;
use regex::Regex;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_13.txt");

type MyPoint = Point<i64>;

#[derive(Debug)]
struct ClawMachine {
    button_a: MyPoint, // Movement vector for button A
    button_b: MyPoint, // Movement vector for button B
    prize: MyPoint,    // Prize location
}

impl FromStr for ClawMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Button A: X\+(?P<ax>\d+), Y\+(?P<ay>\d+)\nButton B: X\+(?P<bx>\d+), Y\+(?P<by>\d+)\nPrize: X=(?P<px>\d+), Y=(?P<py>\d+)").unwrap();

        if let Some(caps) = re.captures(s) {
            return Ok(ClawMachine {
                button_a: MyPoint::new(
                    caps.name("ax").unwrap().as_str().parse().unwrap(),
                    caps.name("ay").unwrap().as_str().parse().unwrap(),
                ),
                button_b: MyPoint::new(
                    caps.name("bx").unwrap().as_str().parse().unwrap(),
                    caps.name("by").unwrap().as_str().parse().unwrap(),
                ),
                prize: MyPoint::new(
                    caps.name("px").unwrap().as_str().parse().unwrap(),
                    caps.name("py").unwrap().as_str().parse().unwrap(),
                ),
            });
        }
        Err(())
    }
}

impl ClawMachine {
    ///
    /// # `is_solvable`
    /// Check if the claw machine is solvable in a maximum of 100 presses of each button.
    ///
    /// ## Returns
    /// - `Some((a, b))` if the machine is solvable, with `a` and `b` being the number of presses of each button.
    fn is_solvable(&self) -> Option<(i64, i64)> {
        // Try all combinations up to 100 presses of each button
        for a in 0..=100 {
            for b in 0..=100 {
                let x = a * self.button_a.x + b * self.button_b.x;
                let y = a * self.button_a.y + b * self.button_b.y;

                if x == self.prize.x && y == self.prize.y {
                    return Some((a, b));
                }
            }
        }

        None
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 13 - Part 1");
    let start = std::time::Instant::now();

    let machines: Vec<ClawMachine> = INPUT.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let mut total_tokens = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = machine.is_solvable() {
            total_tokens += 3 * a_presses + b_presses;
        }
    }

    let duration = start.elapsed();

    println!("Total tokens: {total_tokens}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 13 - Part 2");
    let start = std::time::Instant::now();

    let duration = start.elapsed();

    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    //response_part_2();
}

// Tests ==================================================================================== Tests
