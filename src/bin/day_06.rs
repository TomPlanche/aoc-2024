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

#[derive(Debug, Clone)]
struct Guard {
    position: Point<u32>,
    direction: Directions,
    path: Vec<Point<u32>>, // Stores the history of positions
    steps_taken: usize,
}

impl Guard {
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

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Directions::Up => Directions::Left,
            Directions::Left => Directions::Down,
            Directions::Down => Directions::Right,
            Directions::Right => Directions::Up,
        };
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Directions::Up => Directions::Right,
            Directions::Right => Directions::Down,
            Directions::Down => Directions::Left,
            Directions::Left => Directions::Up,
        };
    }

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

    fn get_next_position(&self) -> Point<u32> {
        match self.direction {
            Directions::Up => Point::new(self.position.x, self.position.y - 1),
            Directions::Down => Point::new(self.position.x, self.position.y + 1),
            Directions::Left => Point::new(self.position.x - 1, self.position.y),
            Directions::Right => Point::new(self.position.x + 1, self.position.y),
        }
    }

    fn direction_char(&self) -> char {
        match self.direction {
            Directions::Up => '^',
            Directions::Down => 'v',
            Directions::Left => '<',
            Directions::Right => '>',
        }
    }
}

// And then modify the Grid struct to use Guard instead of separate position/direction:
#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    obstacles: Vec<Point<u32>>,
    guard: Guard,
}

impl Grid {
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

    // Update display method to use guard
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

    // Previous methods remain the same...
    fn is_obstacle(&self, point: &Point<u32>) -> bool {
        self.obstacles.contains(point)
    }

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
