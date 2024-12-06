///
/// # day_06.rs
/// Code for the day 06 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use aoc_2024::Point;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_06.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

///
/// # Guard
/// Represents a guard entity that can move around the grid and maintains its movement history
#[derive(Debug, Clone)]
struct Guard {
    position: Point<u32>,
    direction: Directions,
    path: Vec<Point<u32>>, // Stores the history of positions
    steps_taken: usize,
}

impl Guard {
    ///
    /// # new
    /// Creates a new Guard instance at the specified position and direction
    ///
    /// ## Arguments
    /// * `position` - Starting position of the guard
    /// * `direction` - Initial direction the guard is facing
    ///
    /// ## Returns
    /// * `Guard` - A new Guard instance with initialized path history
    fn new(position: Point<u32>, direction: Directions) -> Self {
        let mut path = Vec::new();
        path.push(position.clone());

        Guard {
            position,
            direction,
            path,
            steps_taken: 0,
        }
    }

    ///
    /// # turn_right
    /// Rotates the guard 90 degrees clockwise
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Directions::Up => Directions::Right,
            Directions::Right => Directions::Down,
            Directions::Down => Directions::Left,
            Directions::Left => Directions::Up,
        };
    }

    ///
    /// # move_forward
    /// Moves the guard one step forward in their current direction and records the movement
    fn move_forward(&mut self) {
        let new_position = match self.direction {
            Directions::Up => Point::new(self.position.x, self.position.y - 1),
            Directions::Down => Point::new(self.position.x, self.position.y + 1),
            Directions::Left => Point::new(self.position.x - 1, self.position.y),
            Directions::Right => Point::new(self.position.x + 1, self.position.y),
        };

        self.position = new_position;
        self.path.push(self.position.clone());
        self.steps_taken += 1;
    }

    ///
    /// # get_next_position
    /// Calculates the next position without actually moving the guard
    ///
    /// ## Returns
    /// * `Point` - The position the guard would move to if they stepped forward
    fn get_next_position(&self) -> Point<u32> {
        match self.direction {
            Directions::Up => Point::new(self.position.x, self.position.y - 1),
            Directions::Down => Point::new(self.position.x, self.position.y + 1),
            Directions::Left => Point::new(self.position.x - 1, self.position.y),
            Directions::Right => Point::new(self.position.x + 1, self.position.y),
        }
    }

    ///
    /// # direction_char
    /// Converts the guard's current direction to its character representation
    ///
    /// ## Returns
    /// * `char` - ASCII character representing the guard's direction (^, v, <, >)
    fn direction_char(&self) -> char {
        match self.direction {
            Directions::Up => '^',
            Directions::Down => 'v',
            Directions::Left => '<',
            Directions::Right => '>',
        }
    }
}

///
/// # Grid
/// Represents the game grid containing obstacles and a guard
#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    obstacles: Vec<Point<u32>>,
    guard: Guard,
}

impl Grid {
    ///
    /// # from_input
    /// Creates a new Grid instance by parsing a string representation of the grid layout.
    /// The input string should contain characters representing obstacles ('#'),
    /// the guard ('^', 'v', '<', '>'), and empty spaces ('.').
    ///
    /// ## Arguments
    /// * `input` - String containing the grid layout representation
    ///
    /// ## Returns
    /// * `Grid` - A new Grid instance containing the parsed layout and guard position
    fn from_input(input: &str) -> Self {
        let mut obstacles = Vec::new();
        let mut guard_position = Point::new(0, 0);
        let mut guard_direction = Directions::Up;

        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => obstacles.push(Point::new(x as u32, y as u32)),
                    '^' => {
                        guard_position = Point::new(x as u32, y as u32);
                        guard_direction = Directions::Up;
                    }
                    'v' => {
                        guard_position = Point::new(x as u32, y as u32);
                        guard_direction = Directions::Down;
                    }
                    '<' => {
                        guard_position = Point::new(x as u32, y as u32);
                        guard_direction = Directions::Left;
                    }
                    '>' => {
                        guard_position = Point::new(x as u32, y as u32);
                        guard_direction = Directions::Right;
                    }
                    '.' => (), // Empty space
                    _ => panic!("Invalid character in input"),
                }
            }
        }

        Grid {
            width,
            height,
            obstacles,
            guard: Guard::new(guard_position, guard_direction),
        }
    }

    ///
    /// # display
    /// Renders the current state of the grid to stdout
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x as u32, y as u32);
                if p == self.guard.position {
                    print!("{}", self.guard.direction_char());
                } else if self.is_obstacle(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    ///
    /// # is_obstacle
    /// Checks if a given point contains an obstacle
    ///
    /// ## Arguments
    /// * `point` - The point to check for an obstacle
    ///
    /// ## Returns
    /// * `bool` - true if the point contains an obstacle, false otherwise
    fn is_obstacle(&self, point: &Point<u32>) -> bool {
        self.obstacles.contains(point)
    }

    ///
    /// # in_bounds
    /// Verifies if a point is within the grid boundaries
    ///
    /// ## Arguments
    /// * `point` - The point to check
    ///
    /// ## Returns
    /// * `bool` - true if the point is within bounds, false otherwise
    fn in_bounds(&self, point: &Point<u32>) -> bool {
        point.x < self.width as u32 && point.y < self.height as u32
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 06 - Part 1");

    let grid = Grid::from_input(INPUT);
    grid.display();

    let guard = &grid.guard;
    println!("Guard position: {:?}", guard.position);
}

pub fn response_part_2() {
    println!("Day 06 - Part 2");
}

fn main() {
    response_part_1();
    response_part_2();
}
// Tests ==================================================================================== Tests
