///
/// # day_15.rs
/// Code for the day 15 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use aoc_2024::{Direction, Point};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_15.txt");

type Position = Point<usize>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Box {
    position: Position,
    width: usize,
}

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<char>>,
    robot: Position,
    boxes: HashSet<Box>,
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut robot = Position { x: 0, y: 0 };
        let mut boxes = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '@' => {
                        robot = Position { x, y };
                        row.push('.');
                    }
                    'O' => {
                        boxes.insert(Box {
                            position: Position { x, y },
                            width: 1, // Assuming width is 1 for now
                        });
                        row.push('.');
                    }
                    _ => row.push(ch),
                }
            }
            grid.push(row);
        }

        Ok(Warehouse { grid, robot, boxes })
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = self.grid.clone();
        grid[self.robot.y][self.robot.x] = '@';
        for b in &self.boxes {
            grid[b.position.y][b.position.x] = 'O';
        }

        for row in &grid {
            for ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Warehouse {
    ///
    /// # `move_robot`
    /// Move the robot in the given direction.
    ///
    /// ## Arguments
    /// * `direction` - The direction in which the robot should move.
    ///
    /// ## Algorithm
    /// 1. Determine the change in coordinates (dx, dy) based on the direction.
    /// 2. Calculate the new position of the robot.
    /// 3. Initialize a list to keep track of positions to move.
    /// 4. While the new position contains a box:
    ///     a. Calculate the next position for the box.
    ///     b. If the next position is a wall, return without moving.
    ///     c. Add the new position to the list of positions to move.
    /// 5. If the final position is a wall, return without moving.
    /// 6. Move the boxes in reverse order of the positions to move.
    ///     Reverse order is used to ensure that the boxes are moved in the correct order.
    /// 7. Update the robot's position to the first position in the list.
    fn move_robot(&mut self, direction: Direction) {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => unreachable!("Invalid direction"),
        };

        let mut new_robot_pos = Position {
            x: (self.robot.x as isize + dx) as usize,
            y: (self.robot.y as isize + dy) as usize,
        };

        let mut positions_to_move = vec![new_robot_pos];

        // While the new position contains a box
        while self.boxes.iter().any(|b| b.position == new_robot_pos) {
            // Calculate the next position for the box
            new_robot_pos = Position {
                x: (new_robot_pos.x as isize + dx) as usize,
                y: (new_robot_pos.y as isize + dy) as usize,
            };

            // If the next position is a wall, return without moving
            if self.grid[new_robot_pos.y][new_robot_pos.x] == '#' {
                return;
            }

            // Add the new position to the list of positions to move
            positions_to_move.push(new_robot_pos);
        }

        // If the final position is a wall, return without moving
        if self.grid[new_robot_pos.y][new_robot_pos.x] == '#' {
            return;
        }

        // Move the boxes in reverse order of the positions to move
        for pos in positions_to_move.iter().rev() {
            if let Some(box_to_move) = self.boxes.take(&Box {
                position: *pos,
                width: 1,
            }) {
                let new_box_pos = Position {
                    x: (pos.x as isize + dx) as usize,
                    y: (pos.y as isize + dy) as usize,
                };

                self.boxes.insert(Box {
                    position: new_box_pos,
                    width: box_to_move.width,
                });
            }
        }

        self.robot = positions_to_move[0];
    }

    ///
    /// # `execute_moves`
    /// Execute the moves given in the string.
    ///
    /// ## Arguments
    /// * `moves` - The string containing the moves to execute.
    fn execute_moves(&mut self, moves: &str) {
        for c in moves.chars() {
            let direction: Direction = match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => unreachable!("Invalid direction"),
            };

            self.move_robot(direction);
        }
    }

    ///
    /// # `calculate_gps_sum`
    /// Calculate the sum of the GPS coordinates of the boxes.
    ///
    /// ## Returns
    /// * `usize` - The sum of the GPS coordinates of the boxes.
    fn calculate_gps_sum(&self) -> usize {
        self.boxes
            .iter()
            .map(|b| b.position.y * 100 + b.position.x)
            .sum()
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 15 - Part 1");
    let start = Instant::now();

    let mut parts = INPUT.split("\n\n");
    let warehouse_str = parts.next().unwrap();
    let moves = parts.next().unwrap().replace("\n", "");

    let mut warehouse: Warehouse = warehouse_str.parse().unwrap();
    warehouse.execute_moves(&moves);

    let gps_sum = warehouse.calculate_gps_sum();

    let duration = start.elapsed();

    println!("GPS sum: {gps_sum}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 15 - Part 2");
    let start = std::time::Instant::now();

    let duration = start.elapsed();

    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    //response_part_2();
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_part_1() {
        let mut parts = TEST_INPUT.split("\n\n");
        let warehouse_str = parts.next().unwrap();
        let moves = parts.next().unwrap().replace("\n", "");

        let mut warehouse: Warehouse = warehouse_str.parse().unwrap();
        warehouse.execute_moves(&moves);

        let gps_sum = warehouse.calculate_gps_sum();

        assert_eq!(gps_sum, 2028);
    }
}
