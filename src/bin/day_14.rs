///
/// # day_14.rs
/// Code for the day 14 of the Advent of Code challenge year 2024
///
/// This code solves a problem involving robots moving on a grid. Each robot has an initial position and a velocity.
/// The goal is to determine the optimal time to minimize the variance in their positions.
///
/// ## Part 1
/// Calculates the safety factor based on the number of robots in each quadrant after a given number of seconds.
/// The grid is divided into four quadrants, and the safety factor is the product of the number of robots in each quadrant.
///
/// ## Part 2
/// Finds the optimal time to minimize the variance in the robots' positions. The variance is calculated separately for the x and y coordinates.
/// The optimal time is determined using a precomputed inverse of the width modulo the height.
///
/// ## Implementation Details
/// - Uses regex for parsing robot data
/// - Implements modular arithmetic to handle grid wrapping
/// - Uses variance calculation to find the best offset
/// - Handles complex cases including:
///   * Robots with different velocities
///   * Grid wrapping
///
/// ## Key Components
/// - Robot struct: Represents a robot with position and velocity
/// - Robots struct: Manages a collection of robots and provides methods for movement and variance calculation
/// - position_after: Computes the position of a robot after a given number of seconds
/// - find_best_offset: Finds the best offset to minimize variance
/// - move_instances: Moves robots and returns their new positions
///
// Imports  ==============================================================================  Imports
use aoc_2024::calculate_variance;

use regex::Regex;
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_14.txt");

#[derive(Debug, Clone)]
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

struct Robots {
    instances: Vec<Robot>,
}

impl Robots {
    fn new(instances: Vec<Robot>) -> Self {
        Robots { instances }
    }

    ///
    /// # `find_best_offset`
    /// Find the best offset to minimize the variance in the robots' positions.
    /// The method iterates over possible offsets and calculates the variance for each offset.
    ///
    /// ## Arguments
    /// * `modulo` - The modulo value for the grid.
    /// * `use_x` - A boolean flag indicating whether to use the x-coordinate for variance calculation.
    ///
    /// ## Returns
    /// * `i32` - The best offset to minimize the variance.
    fn find_best_offset(&self, modulo: i32, use_x: bool) -> i32 {
        let mut best_variance = f64::MAX;
        let mut best_offset = 0;

        for offset in 0..modulo {
            let positions: Vec<_> = self.move_instances(modulo, modulo, offset).collect();

            let variance = if use_x {
                calculate_variance(&positions)
            } else {
                calculate_variance(&positions.iter().map(|&(x, y)| (y, x)).collect::<Vec<_>>())
            };

            if variance < best_variance {
                best_variance = variance;
                best_offset = offset;
            }
        }

        best_offset
    }

    ///
    /// # `move_instances`
    /// Move the robots and return their new positions.
    ///
    /// ## Arguments
    /// * `width` - The width of the grid.
    /// * `height` - The height of the grid.
    /// * `steps` - The number of steps to move the robots.
    fn move_instances<'a>(
        &'a self,
        width: i32,
        height: i32,
        steps: i32,
        // 'a is the lifetime of the iterator, Rust witchcraft to make it work
    ) -> impl Iterator<Item = (i32, i32)> + 'a {
        let steps_x = steps % width;
        let steps_y = steps % height;

        self.instances.iter().map(move |robot| {
            let new_x = (robot.position.0 + steps_x * robot.velocity.0).rem_euclid(width);
            let new_y = (robot.position.1 + steps_y * robot.velocity.1).rem_euclid(height);
            (new_x, new_y)
        })
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

    let robots = INPUT
        .trim()
        .lines()
        .map(|line| line.parse::<Robot>().unwrap())
        .collect::<Vec<_>>();

    let width: i32 = 101;
    let height: i32 = 103;

    let robots = Robots::new(robots);
    let best_offset_x = robots.find_best_offset(width, true) as i64;
    let best_offset_y = robots.find_best_offset(height, false) as i64;

    // The inverse of the width modulo the height is calculated to optimize the time calculation.
    // The inverse is used to align the best offsets for the x and y coordinates.
    // It's calculated by finding the value of `i` such that `(i * width) % height == 1`.
    let inv_w = i64::from((0..height).find(|&i| (i * width) % height == 1).unwrap());

    // The optimal time is calculated using a formula that combines the best offsets for the x and y coordinates.
    //
    //  The formula `(best_offset_x + INV_W * (best_offset_y - best_offset_x) * i64::from(width))` computes the optimal time in a way that
    // aligns the best offsets for both coordinates.
    //
    //  `rem_euclid` is used to ensure the result is within the valid range of time (0 to `width * height - 1`).
    let optimal_time = best_offset_x + inv_w * (best_offset_y - best_offset_x) * i64::from(width);
    let optimal_time_within_bounds = optimal_time.rem_euclid(i64::from(width * height));

    let duration = start.elapsed();

    // print the robots' positions after `optimal_time_within_bounds` time.
    let positions = robots
        .move_instances(width, height, optimal_time_within_bounds as i32)
        .collect::<Vec<_>>();

    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for (x, y) in positions {
        grid[y as usize][x as usize] = '@';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Optimal time: {optimal_time_within_bounds}");
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
