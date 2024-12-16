///
/// # day_16.rs
/// Code for the day 16 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::{cmp::Ordering, collections::BinaryHeap, str::FromStr};

use aoc_2024::{Direction, Point};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_16.txt");

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

type MyPoint = Point<usize>;

#[derive(Debug)]
struct Maze {
    grid: Vec<Vec<Tile>>,
    start: MyPoint,
    end: MyPoint,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut start = MyPoint::new(0, 0);
        let mut end = MyPoint::new(0, 0);

        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' => row.push(Tile::Wall),
                    '.' => row.push(Tile::Empty),
                    'S' => {
                        row.push(Tile::Start);

                        start = MyPoint::from((i, j));
                    }
                    'E' => {
                        row.push(Tile::End);
                        end = MyPoint::from((i, j));
                    }
                    _ => panic!("Invalid character in maze"),
                }
            }
            grid.push(row);
        }

        Ok(Maze { grid, start, end })
    }
}

#[derive(Eq, PartialEq)]
struct State {
    cost: i32,
    position: MyPoint,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Maze {
    ///
    /// # `find_shortest_path`
    /// Find the shortest path from the start to the end of the maze.
    ///
    /// ## Algorithm
    /// Uses Dijkstra's algorithm with a modified cost structure:
    /// 1. Maintains a priority queue of States (position, direction, cost)
    /// 2. Each state represents a position in the maze and the direction facing
    /// 3. For each state, three possible moves are considered:
    ///    - Move forward (cost +1)
    ///    - Turn clockwise (cost +1000)
    ///    - Turn counterclockwise (cost +1000)
    /// 4. Uses a visited set to avoid cycles and redundant exploration
    /// 5. Higher turning cost (1000) prioritizes straight paths over turns
    /// 6. Terminates when reaching the end position or exhausting all possibilities
    /// 7. Returns the minimum total cost to reach the end
    ///
    /// ## Returns
    /// * `Some(i32)`: The cost of the shortest path
    fn find_shortest_path(&self) -> Option<i32> {
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        // Track visited states to avoid cycles
        let mut visited = std::collections::HashSet::new();

        // Priority queue for Dijkstra's algorithm
        let mut queue = BinaryHeap::new();

        // Start facing right (east) as per instructions
        queue.push(State {
            cost: 0,
            position: self.start,
            direction: Direction::Right,
        });

        while let Some(State {
            cost,
            position,
            direction,
        }) = queue.pop()
        {
            // Found the end
            if position == self.end {
                return Some(cost);
            }

            // Skip if we've seen this state before
            let state = (position, direction);
            if !visited.insert(state) {
                continue;
            }

            let (row, col) = position.into();
            let (dy, dx) = direction.into();

            // Try moving forward
            let new_row = (row as i32 + dy as i32) as usize;
            let new_col = (col as i32 + dx as i32) as usize;

            if new_row < rows && new_col < cols && self.grid[new_row][new_col] != Tile::Wall {
                queue.push(State {
                    cost: cost + 1, // Add 1 for moving forward
                    position: (new_row, new_col).into(),
                    direction,
                });
            }

            // Try turning clockwise
            let clockwise = direction.turn_clockwise();
            queue.push(State {
                cost: cost + 1000, // Add 1000 for turning
                position,
                direction: clockwise,
            });

            // Try turning counter-clockwise
            let counter_clockwise = direction.turn_counterclockwise();
            queue.push(State {
                cost: cost + 1000, // Add 1000 for turning
                position,
                direction: counter_clockwise,
            });
        }

        None // No path found
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 16 - Part 1");
    let start = std::time::Instant::now();

    let maze = Maze::from_str(INPUT).unwrap();
    let result = maze.find_shortest_path().unwrap();

    let duration = start.elapsed();

    println!("Lowest possible score: {}", result);
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 16 - Part 2");
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

    const EXAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_example_part_1() {
        let maze = Maze::from_str(EXAMPLE).unwrap();

        let result = maze.find_shortest_path().unwrap();

        assert_eq!(result, 14);
    }
}
