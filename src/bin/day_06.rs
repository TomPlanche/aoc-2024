///
/// # `day_06.rs`
/// Code for the day 06 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use aoc_2024::{Direction, Point};
use indicatif::ProgressBar;
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_06.txt");

///
/// # `Guard`
/// Represents a guard entity that can move around the grid and maintains its movement history
#[derive(Debug, Clone)]
struct Guard {
    position: Point<i32>,
    direction: Direction,
    path: Vec<Point<i32>>, // Stores the history of positions
    steps_taken: usize,
}

impl Guard {
    ///
    /// # `new`
    /// Creates a new Guard instance at the specified position and direction
    ///
    /// ## Arguments
    /// * `position` - Starting position of the guard
    /// * `direction` - Initial direction the guard is facing
    ///
    /// ## Returns
    /// * `Guard` - A new Guard instance with initialized path history
    fn new(position: Point<i32>, direction: Direction) -> Self {
        let path = vec![position];

        Guard {
            position,
            direction,
            path,
            steps_taken: 0,
        }
    }

    ///
    /// # `direction_char`
    /// Returns the character representation of the guard's current direction
    ///
    /// ## Returns
    /// * `char` - The character representing the guard's direction
    #[allow(dead_code)]
    fn direction_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            _ => unreachable!(),
        }
    }

    ///
    /// # `turn_right`
    /// Rotates the guard 90 degrees clockwise
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            _ => unreachable!(),
        };
    }

    ///
    /// # `move_forward`
    /// Moves the guard one step forward in their current direction and records the movement
    fn move_forward(&mut self) {
        let new_position = match self.direction {
            Direction::Up => Point::new(self.position.x, self.position.y - 1),
            Direction::Down => Point::new(self.position.x, self.position.y + 1),
            Direction::Left => Point::new(self.position.x - 1, self.position.y),
            Direction::Right => Point::new(self.position.x + 1, self.position.y),
            _ => unreachable!(),
        };

        self.position = new_position;
        self.path.push(self.position);
        self.steps_taken += 1;
    }

    ///
    /// # `get_next_position`
    /// Calculates the next position without actually moving the guard
    ///
    /// ## Returns
    /// * `Point` - The position the guard would move to if they stepped forward
    fn get_next_position(&self) -> Point<i32> {
        match self.direction {
            Direction::Up => Point::new(self.position.x, self.position.y - 1),
            Direction::Down => Point::new(self.position.x, self.position.y + 1),
            Direction::Left => Point::new(self.position.x - 1, self.position.y),
            Direction::Right => Point::new(self.position.x + 1, self.position.y),
            _ => unreachable!(),
        }
    }
}

///
/// # `Grid`
/// Represents the game grid containing obstacles and a guard
#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    obstacles: Vec<Point<i32>>,
    guard: Guard,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obstacles = Vec::new();
        let mut guard_position = Point::new(0, 0);
        let mut guard_direction = Direction::Up;

        let mut width = 0;
        let mut height = 0;

        for (y, line) in s.lines().enumerate() {
            height += 1;
            width = line.len();

            for (x, c) in line.chars().enumerate() {
                let p = Point::new(i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
                match c {
                    '#' => obstacles.push(p),
                    '^' => {
                        guard_position = p;
                        guard_direction = Direction::Up;
                    }
                    'v' => {
                        guard_position = p;
                        guard_direction = Direction::Down;
                    }
                    '<' => {
                        guard_position = p;
                        guard_direction = Direction::Left;
                    }
                    '>' => {
                        guard_position = p;
                        guard_direction = Direction::Right;
                    }
                    _ => {}
                }
            }
        }

        Ok(Grid {
            width,
            height,
            obstacles,
            guard: Guard::new(guard_position, guard_direction),
        })
    }
}

impl Grid {
    ///
    /// # `display`
    /// Renders the current state of the grid to stdout
    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
                if p == self.guard.position {
                    print!("{}", self.guard.direction_char());
                } else if self.is_obstacle(p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    ///
    /// # `is_obstacle`
    /// Checks if a given point contains an obstacle
    ///
    /// ## Arguments
    /// * `point` - The point to check for an obstacle
    ///
    /// ## Returns
    /// * `bool` - true if the point contains an obstacle, false otherwise
    fn is_obstacle(&self, point: Point<i32>) -> bool {
        self.obstacles.contains(&point)
    }

    ///
    /// # `in_bounds`
    /// Verifies if a point is within the grid boundaries
    ///
    /// ## Arguments
    /// * `point` - The point to check
    ///
    /// ## Returns
    /// * `bool` - true if the point is within bounds, false otherwise
    fn in_bounds(&self, point: Point<i32>) -> bool {
        point.x < i32::try_from(self.width).unwrap()
            && point.y < i32::try_from(self.height).unwrap()
    }

    ///
    /// # `can_move_to`
    /// Checks if a point is within bounds and not an obstacle
    ///
    /// ## Arguments
    /// * `point` - The point to check
    ///
    /// ## Returns
    /// * `bool` - true if the point is within bounds and not an obstacle, false otherwise
    fn can_move_to(&self, point: Point<i32>) -> bool {
        self.in_bounds(point) && !self.is_obstacle(point)
    }

    ///
    /// # `simulate_guard_movement`
    /// Simulates the guard's movement until it leaves the mapped area
    ///
    /// ## Returns
    /// * `usize` - The number of distinct positions visited by the guard
    fn simulate_guard_movement(&mut self) -> usize {
        let mut visited = std::collections::HashSet::new();
        visited.insert(self.guard.position);

        loop {
            let next_position = self.guard.get_next_position();

            // Check if guard would leave the mapped area
            if !self.in_bounds(next_position) {
                break;
            }

            if self.can_move_to(next_position) {
                self.guard.move_forward();
                visited.insert(self.guard.position);
            } else {
                self.guard.turn_right();
            }

            // Optional safety check
            if self.guard.steps_taken > self.width * self.height * 4 {
                println!("Guard has taken too many steps, ending simulation");
                break;
            }
        }

        visited.len()
    }

    ///
    /// # `simulate_with_obstacle`
    /// Simulates the guard's movement with an additional obstacle and checks if it creates a loop.
    ///
    /// ## Arguments
    /// * `obstacle` - The position of the obstacle to add
    ///
    /// ## Returns
    /// * `Option<bool>` - Some(true) if the obstacle creates a loop, Some(false) if it doesn't, None if the obstacle is invalid
    fn simulate_with_obstacle(&mut self, obstacle: Point<i32>) -> Option<bool> {
        if obstacle == self.guard.position || self.is_obstacle(obstacle) {
            return None;
        }

        let mut visited_states = std::collections::HashSet::new();
        let mut temp_obstacles = self.obstacles.clone();
        temp_obstacles.push(obstacle);

        let mut current_pos = self.guard.position;
        let mut current_dir = self.guard.direction;

        loop {
            // Create a unique state representation
            let state = (current_pos, current_dir);
            if !visited_states.insert(state) {
                // Found a loop
                return Some(true);
            }

            let next_pos = match current_dir {
                Direction::Up => Point::new(current_pos.x, current_pos.y - 1),
                Direction::Down => Point::new(current_pos.x, current_pos.y + 1),
                Direction::Left => Point::new(current_pos.x - 1, current_pos.y),
                Direction::Right => Point::new(current_pos.x + 1, current_pos.y),
                _ => unreachable!(),
            };

            // Check if out of bounds
            if !self.in_bounds(next_pos) {
                return Some(false);
            }

            // Check if hitting obstacle (including the new one)
            if temp_obstacles.contains(&next_pos) {
                current_dir = match current_dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    _ => unreachable!(),
                };
            } else {
                current_pos = next_pos;
            }

            // Safety check for infinite loops
            if visited_states.len() > self.width * self.height * 4 {
                return Some(false);
            }
        }
    }

    ///
    /// # `count_possible_loop_positions`
    /// Counts the number of possible loop positions that can be added to the grid
    ///
    /// ## Returns
    /// * `usize` - The number of possible loop positions
    fn count_possible_loop_positions(&mut self) -> usize {
        let mut count = 0;

        // First simulate the guard's movement to get potential positions
        let mut potential_positions = std::collections::HashSet::new();
        let mut current_pos = self.guard.position;
        let mut current_dir = self.guard.direction;

        // Get all positions the guard could potentially visit
        loop {
            let next_pos = match current_dir {
                Direction::Up => Point::new(current_pos.x, current_pos.y - 1),
                Direction::Down => Point::new(current_pos.x, current_pos.y + 1),
                Direction::Left => Point::new(current_pos.x - 1, current_pos.y),
                Direction::Right => Point::new(current_pos.x + 1, current_pos.y),
                _ => unreachable!(),
            };

            if !self.in_bounds(next_pos) {
                break;
            }

            if self.is_obstacle(next_pos) {
                current_dir = match current_dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    _ => unreachable!(),
                };
            } else {
                potential_positions.insert(next_pos);
                current_pos = next_pos;
            }

            if potential_positions.len() > self.width * self.height {
                break;
            }
        }

        let pb = ProgressBar::new(potential_positions.len() as u64);

        // Only test positions that are part of the guard's potential path
        for test_point in potential_positions {
            pb.inc(1);
            if let Some(creates_loop) = self.simulate_with_obstacle(test_point) {
                if creates_loop {
                    count += 1;
                }
            }
        }

        pb.finish_with_message("done");
        count
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 06 - Part 1");
    let start = std::time::Instant::now();

    let mut grid: Grid = INPUT.parse().unwrap();
    let visited = grid.simulate_guard_movement();

    let duration = start.elapsed();

    println!("Number of distinct positions visited: {visited}");
    println!("Duration: {duration:?}\n");
}

pub fn response_part_2() {
    println!("Day 06 - Part 2");
    let start = std::time::Instant::now();

    let mut grid: Grid = INPUT.parse().unwrap();
    let loop_positions = grid.count_possible_loop_positions();

    let duration = start.elapsed();

    println!("Number of possible positions for new obstacle: {loop_positions}");
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

    const TEST_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_example_distinct_positions() {
        let mut grid: Grid = TEST_INPUT.parse().unwrap();
        let visited = grid.simulate_guard_movement();

        grid.display();

        assert_eq!(visited, 41);
    }

    #[test]
    fn test_example_loop_positions() {
        let mut grid: Grid = TEST_INPUT.parse().unwrap();
        let loop_positions = grid.count_possible_loop_positions();

        assert_eq!(loop_positions, 6);
    }
}
