///
/// # day_13.rs
/// Code for the day 13 of the Advent of Code challenge year 2024
///
/// ## Part 1
/// The part 1 is super easy with just brute force. See the first commit for the brute force code.
///
/// ## Part 2
/// This part is unsolvable with brute force. We need to think...
/// The problem is a basic linear system of equations.
/// With the prize coordinates being (x, y), the movement vectors for the buttons being (a_x, a_y) and (b_x, b_y),
/// we have the following system:
/// a_x * a + b_x * b = x
/// a_y * a + b_y * b = y
///
/// I did some research and found that this is a classic problem in linear algebra.
/// I watches the following video to understand the solution:
/// - https://www.youtube.com/watch?v=jBsC34PxzoM
/// - https://www.youtube.com/watch?v=vXqlIOX2itM
// Imports  ==============================================================================  Imports
use aoc_2024::Point;
use regex::Regex;
use std::str::FromStr;

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
    /// # `is_solvable`
    /// Checks if the prize coordinates (offset by a large value) can be reached using the two buttons.
    ///
    /// ## Matrix Formulation
    /// The problem can be expressed as the matrix equation AX = B:
    ///
    /// [button_a.x  button_b.x] [a] = [prize_x]
    /// [button_a.y  button_b.y] [b]   [prize_y]
    ///
    /// Where:
    /// - A is the 2×2 coefficient matrix of button coordinates
    /// - X is the 2×1 vector of button presses (a,b) we're solving for
    /// - B is the 2×1 vector of target prize coordinates
    ///
    /// ## Cramer's Rule Application
    /// For a 2×2 system, Cramer's rule gives solutions:
    ///
    /// a = det(A₁)/det(A)  where A₁ = [prize_x    button_b.x]
    ///                                [prize_y    button_b.y]
    ///
    /// b = det(A₂)/det(A)  where A₂ = [button_a.x    prize_x]
    ///                                [button_a.y    prize_y]
    ///
    /// det(A) = |button_a.x  button_b.x| = button_a.x * button_b.y - button_a.y * button_b.x
    ///          |button_a.y  button_b.y|
    ///
    /// ## Arguments
    /// * `offset` - Value added to prize coordinates to check solvability at different positions
    ///
    /// ## Returns
    /// * `Some((a, b))` if solution exists, where a,b are integer button presses
    /// * `None` if no solution exists (det(A) = 0 or solution doesn't verify)
    fn is_solvable(&self, offset: i64) -> Option<(i64, i64)> {
        // Offset prize coordinates
        let prize_x = self.prize.x + offset;
        let prize_y = self.prize.y + offset;

        // Calculate det(A) = |button_a.x  button_b.x|
        //                    |button_a.y  button_b.y|
        let det = self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;

        // If det(A) = 0, matrix is singular (buttons are linearly dependent)
        // meaning no unique solution exists
        if det == 0 {
            return None;
        }

        // Calculate a using det(A₁)/det(A) where:
        // det(A₁) = |prize_x    button_b.x|
        //           |prize_y    button_b.y|
        let a = (prize_x * self.button_b.y - prize_y * self.button_b.x) / det;

        // Calculate b using det(A₂)/det(A) where:
        // det(A₂) = |button_a.x    prize_x|
        //           |button_a.y    prize_y|
        let b = (self.button_a.x * prize_y - self.button_a.y * prize_x) / det;

        // Verify solution by multiplying original matrix equation:
        // [button_a.x  button_b.x] [a] ?= [prize_x]
        // [button_a.y  button_b.y] [b]    [prize_y]
        let check_x = self.button_a.x * a + self.button_b.x * b;
        let check_y = self.button_a.y * a + self.button_b.y * b;

        // Return solution only if verification passes exactly
        if check_x == prize_x && check_y == prize_y {
            Some((a, b))
        } else {
            None
        }
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 13 - Part 1");
    let start = std::time::Instant::now();

    let machines: Vec<ClawMachine> = INPUT.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let mut total_tokens = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = machine.is_solvable(0) {
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

    let machines: Vec<ClawMachine> = INPUT.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let mut total_tokens = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = machine.is_solvable(10000000000000) {
            total_tokens += 3 * a_presses + b_presses;
        }
    }

    let duration = start.elapsed();

    println!("Total tokens: {total_tokens}");
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

    const BUTTONS_1: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    const BUTTONS_2: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279";

    #[test]
    fn test_part_1() {
        let machines: Vec<ClawMachine> = BUTTONS_1
            .split("\n\n")
            .map(|s| s.parse().unwrap())
            .collect();

        let mut total_tokens = 0;

        for machine in machines {
            if let Some((a_presses, b_presses)) = machine.is_solvable(0) {
                total_tokens += 3 * a_presses + b_presses;
            }
        }

        assert_eq!(total_tokens, 480);
    }
}
