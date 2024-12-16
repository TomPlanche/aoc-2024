///
/// # day_15.rs
/// Code for the day 15 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use aoc_2024::Direction;
use std::str::FromStr;
use std::time::Instant;
use std::{fmt, mem};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_15.txt");

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Empty,
    Wall,
    Object,
    Robot,
    BoxLeft,
    BoxRight,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Object,
            '@' => Self::Robot,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => Self::Empty,
        }
    }
}

impl Tile {
    fn double(self) -> impl Iterator<Item = Self> {
        match self {
            Tile::Empty => [Tile::Empty, Tile::Empty].into_iter(),
            Tile::Wall => [Tile::Wall, Tile::Wall].into_iter(),
            Tile::Object => [Tile::BoxLeft, Tile::BoxRight].into_iter(),
            Tile::Robot => [Tile::Robot, Tile::Empty].into_iter(),
            Tile::BoxLeft | Tile::BoxRight => panic!(),
        }
    }
}

impl From<Tile> for char {
    fn from(val: Tile) -> Self {
        match val {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Object => 'O',
            Tile::Robot => '@',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<Tile>>,
    robot: (usize, usize),
    width: usize,
    height: usize,
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut robot = (0, 0);

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                if ch == '@' {
                    robot = (y, x)
                }

                row.push(ch.into());
            }
            grid.push(row);
        }

        let width = grid[0].len();
        let height = grid.len();

        Ok(Warehouse {
            grid,
            robot,
            width,
            height,
        })
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for tile in row {
                write!(f, "{}", char::from(*tile))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Warehouse {
    fn scale_width(&mut self) {
        let mut temp: Vec<Vec<Tile>> = Vec::new();
        mem::swap(&mut self.grid, &mut temp);

        self.grid = temp
            .into_iter()
            .map(|row| row.into_iter().flat_map(Tile::double).collect())
            .collect();

        self.width *= 2;
        self.robot.1 *= 2;
    }

    fn move_robot(&mut self, direction: Direction) {
        let (row, col) = self.robot;

        if self.can_move_tile(row, col, direction) {
            self.move_tile(row, col, direction);
            self.robot = self.robot + direction;
        }
    }

    fn move_tile(&mut self, row: usize, col: usize, direction: Direction) {
        let (next_row, next_col) = (row, col) + direction;
        let next_tile = self.grid[next_row][next_col];

        match next_tile {
            Tile::Empty => {
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Tile::Empty;
            }
            Tile::Object => {
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Tile::Empty;
            }
            Tile::BoxRight => {
                self.move_tile(next_row, next_col - 1, direction);
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Tile::Empty;
            }
            Tile::BoxLeft => {
                self.move_tile(next_row, next_col + 1, direction);
                self.move_tile(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Tile::Empty;
            }
            Tile::Wall => panic!(),
            Tile::Robot => panic!(),
        }
    }

    fn can_move_tile(&self, row: usize, col: usize, direction: Direction) -> bool {
        let (next_row, next_col) = (row, col) + direction;
        let next_tile = self.grid[next_row][next_col];

        match next_tile {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Object => self.can_move_tile(next_row, next_col, direction),
            Tile::BoxLeft => {
                if direction == Direction::Left {
                    self.can_move_tile(next_row, next_col, direction)
                } else if direction == Direction::Right {
                    self.can_move_tile(next_row, next_col + 1, direction)
                } else {
                    self.can_move_tile(next_row, next_col + 1, direction)
                        && self.can_move_tile(next_row, next_col, direction)
                }
            }
            Tile::BoxRight => {
                if direction == Direction::Right {
                    self.can_move_tile(next_row, next_col, direction)
                } else if direction == Direction::Left {
                    self.can_move_tile(next_row, next_col - 1, direction)
                } else {
                    self.can_move_tile(next_row, next_col - 1, direction)
                        && self.can_move_tile(next_row, next_col, direction)
                }
            }
            Tile::Robot => panic!(),
        }
    }

    fn gps_coordinate(row: usize, col: usize) -> usize {
        row * 100 + col
    }

    fn sum_gps_coordinates(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.grid[row][col] == Tile::Object || self.grid[row][col] == Tile::BoxLeft {
                    sum += Self::gps_coordinate(row, col);
                }
            }
        }

        sum
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 15 - Part 1");
    let start = Instant::now();

    let mut parts = INPUT.split("\n\n");
    let warehouse_str = parts.next().unwrap();
    let moves: Vec<Direction> = parts
        .next()
        .unwrap()
        .replace("\n", "")
        .chars()
        .map(Direction::from)
        .collect();

    let mut warehouse: Warehouse = warehouse_str.parse().unwrap();

    for direction in moves {
        warehouse.move_robot(direction);
    }

    let sum = warehouse.sum_gps_coordinates();

    let duration = start.elapsed();

    println!("Sum of GPS coordinates: {}", sum);
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 15 - Part 2");
    let start = std::time::Instant::now();

    let mut parts = INPUT.split("\n\n");
    let warehouse = parts.next().unwrap();
    let moves = parts.next().unwrap();

    let mut warehouse = warehouse.parse::<Warehouse>().unwrap();
    let moves: Vec<Direction> = moves
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect();

    warehouse.scale_width();

    for direction in moves {
        warehouse.move_robot(direction);
    }

    let sum = warehouse.sum_gps_coordinates();

    let duration = start.elapsed();

    println!("Sum of GPS coordinates: {}", sum);
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
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part_1() {
        let mut parts = TEST_INPUT.split("\n\n");

        let mut warehouse: Warehouse = parts.next().unwrap().parse().unwrap();
        let moves: Vec<Direction> = parts
            .next()
            .unwrap()
            .replace("\n", "")
            .chars()
            .map(Direction::from)
            .collect();

        for direction in moves {
            warehouse.move_robot(direction);
        }

        assert_eq!(warehouse.sum_gps_coordinates(), 10092);
    }

    #[test]
    fn test_part_2() {
        let mut parts = TEST_INPUT.split("\n\n");

        let mut warehouse: Warehouse = parts.next().unwrap().parse().unwrap();
        let moves: Vec<Direction> = parts
            .next()
            .unwrap()
            .replace("\n", "")
            .chars()
            .map(Direction::from)
            .collect();

        warehouse.scale_width();

        println!("{}", warehouse);

        for direction in moves {
            println!("{:?}", direction);
            warehouse.move_robot(direction);
            println!("{}", warehouse);
        }

        assert_eq!(warehouse.sum_gps_coordinates(), 9021);
    }
}
