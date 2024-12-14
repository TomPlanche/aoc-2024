use std::str::FromStr;

///
/// # day_14.rs
/// Code for the day 14 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use regex::Regex;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_14.txt");

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"p=(?P<px>-?\d+),(?P<py>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)").unwrap();

        let caps = re.captures(s).unwrap();
        let px = caps["px"].parse().unwrap();
        let py = caps["py"].parse().unwrap();
        let vx = caps["vx"].parse().unwrap();
        let vy = caps["vy"].parse().unwrap();

        Ok(Robot::new((px, py), (vx, vy)))
    }
}

impl Robot {
    fn new(position: (i32, i32), velocity: (i32, i32)) -> Self {
        Robot { position, velocity }
    }

    ///
    /// # `position_after`
    /// Compute the position of the robot after a certain amount of seconds.
    ///
    /// ## Arguments
    /// * `seconds` - The amount of seconds to wait.
    /// * `width` - The width of the grid.
    /// * `height` - The height of the grid.
    ///
    /// ## Returns
    /// * `(i32, i32)` - The position of the robot after the given amount of seconds.
    fn position_after(&self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
        // `rem_euclid` is used to handle negative values.
        // It is equivalent to `(x % y + y) % y` but more efficient.
        let x = (self.position.0 + self.velocity.0 * seconds).rem_euclid(width);
        let y = (self.position.1 + self.velocity.1 * seconds).rem_euclid(height);

        (x, y)
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 14 - Part 1");
    let start = std::time::Instant::now();

    let robots = INPUT
        .trim()
        .lines()
        .map(|line| line.parse::<Robot>().unwrap())
        .collect::<Vec<_>>();

    let width = 101;
    let height = 103;
    let seconds = 100;

    let mut quadrant_counts = [0; 4];

    for robot in robots {
        let (x, y) = robot.position_after(seconds, width, height);

        // The center of the grid is not considered.
        if x != width / 2 && y != height / 2 {
            // Compute the quadrant of the robot.
            let quadrant = if x < width / 2 {
                // x < width / 2 corresponds to the left side of the grid.
                if y < height / 2 {
                    // y < height / 2 corresponds to the top side of the grid.
                    0
                } else {
                    // y >= height / 2 corresponds to the bottom side of the grid.
                    2
                }
            } else {
                // x >= width / 2 corresponds to the right side of the grid.
                if y < height / 2 {
                    // y < height / 2 corresponds to the top side of the grid.
                    1
                } else {
                    // y >= height / 2 corresponds to the bottom side of the grid.
                    3
                }
            };

            quadrant_counts[quadrant] += 1;
        }
    }

    let safety_factor = quadrant_counts.iter().product::<i32>();

    let duration = start.elapsed();

    println!("Safety factor: {safety_factor}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 14 - Part 2");
    let start = std::time::Instant::now();

    let duration = start.elapsed();

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

    const TEST_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_1() {
        let robots = TEST_INPUT
            .trim()
            .lines()
            .map(|line| line.parse::<Robot>().unwrap())
            .collect::<Vec<_>>();

        let width = 11;
        let height = 7;
        let seconds = 100;

        let mut quadrant_counts = [0; 4];

        for robot in robots {
            let (x, y) = robot.position_after(seconds, width, height);
            if x != width / 2 && y != height / 2 {
                let quadrant = if x < width / 2 {
                    if y < height / 2 {
                        0
                    } else {
                        2
                    }
                } else {
                    if y < height / 2 {
                        1
                    } else {
                        3
                    }
                };
                quadrant_counts[quadrant] += 1;
            }
        }

        let safety_factor = quadrant_counts.iter().product::<i32>();

        assert_eq!(safety_factor, 12);
    }
}
