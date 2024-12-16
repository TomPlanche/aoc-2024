///
/// # day_16.rs
/// Code for the day 16 of the Advent of Code challenge year 2024
///
/// ## Implementation Notes
/// The code uses `FnvHashSet` instead of standard `HashSet` for performance optimization:
///
/// 1. `FnvHashSet` uses the Fowler-Noll-Vo (FNV) hashing algorithm, which is:
///    - Simpler and faster than the default SipHash algorithm used by `HashSet`
///    - Optimized for small keys (like the Point type with just two integers)
///    - Has good distribution properties for integer-based keys
///
/// 2. Standard `HashSet` uses SipHash which:
///    - Is cryptographically strong
///    - Provides protection against hash-flooding attacks
///    - Has more overhead due to its security features
///
/// In this pathfinding context:
/// - Hash-flooding protection isn't needed
/// - We're dealing with simple point coordinates as keys
/// - We're potentially inserting/checking many points during pathfinding
/// - Performance is more important than hash collision resistance
// Imports  ==============================================================================  Imports
use aoc_2024::{Direction, Point};
use fnv::FnvHashSet;
use std::{cmp::Ordering, collections::BinaryHeap, str::FromStr};

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
    path: Vec<MyPoint>, // Add this field
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
    /// Uses a modified Dijkstra's algorithm with the following approach:
    ///
    /// 1. Maintains a priority queue of States, where each State contains:
    ///    - Current cost
    ///    - Current position
    ///    - Current direction
    ///    - Path taken so far
    ///
    /// 2. Uses a 3D visited array to track costs for each position and direction
    ///    - Dimensions: [row][col][direction]
    ///
    /// 3. Movement rules:
    ///    - Forward movement costs 1
    ///    - Turning (clockwise or counterclockwise) costs 1000
    ///    - Cannot move through walls
    ///
    /// 4. For each position:
    ///    - Tries moving forward in current direction
    ///    - Tries turning both clockwise and counterclockwise
    ///    - Updates visited states if a better cost is found
    ///
    /// 5. Collects all paths that reach the end with the minimum cost
    ///    - Stores unique tiles from these paths in a HashSet
    ///
    /// 6. Returns tuple containing:
    ///    - The minimum cost to reach the end
    ///    - The total number of unique tiles visited across all optimal paths
    ///
    /// ## Returns
    /// * `Some(i32)`: The cost of the shortest path
    fn find_all_best_paths(&self) -> Option<(i32, usize)> {
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        let mut visited = vec![vec![[None; 4]; cols]; rows]; // Track visited states with costs
        let mut queue = BinaryHeap::new();
        let mut path_tiles = FnvHashSet::default();
        let mut final_cost = None;

        // Start facing right
        queue.push(State {
            cost: 0,
            position: self.start,
            direction: Direction::Right,
            path: vec![self.start],
        });

        while let Some(State {
            cost,
            position,
            direction,
            path,
        }) = queue.pop()
        {
            let (row, col) = position.into();

            // Found the end
            if position == self.end {
                if final_cost.is_none() {
                    final_cost = Some(cost);
                }

                if final_cost == Some(cost) {
                    path_tiles.extend(path);
                }
                continue;
            }

            // Check if we've seen this state with a better cost
            if let Some(prev_cost) = visited[row][col][direction as usize] {
                if prev_cost < cost {
                    continue;
                }
                if prev_cost == cost && path_tiles.contains(&position) {
                    path_tiles.extend(path);
                    continue;
                }
            }

            // Update visited state
            visited[row][col][direction as usize] = Some(cost);

            let (dy, dx) = direction.into();

            // Try moving forward
            let new_row = (row as i32 + dy as i32) as usize;
            let new_col = (col as i32 + dx as i32) as usize;

            if new_row < rows && new_col < cols && self.grid[new_row][new_col] != Tile::Wall {
                let mut new_path = path.clone();
                new_path.push((new_row, new_col).into());

                queue.push(State {
                    cost: cost + 1,
                    position: (new_row, new_col).into(),
                    direction,
                    path: new_path,
                });
            }

            // Try turning
            for new_direction in [
                direction.turn_clockwise(),
                direction.turn_counterclockwise(),
            ] {
                let new_path = path.clone();
                queue.push(State {
                    cost: cost + 1000,
                    position,
                    direction: new_direction,
                    path: new_path,
                });
            }
        }

        final_cost.map(|cost| (cost, path_tiles.len()))
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 16 - Part 1");
    let start = std::time::Instant::now();

    let maze = Maze::from_str(INPUT).unwrap();
    let result = maze.find_all_best_paths().unwrap();

    let duration = start.elapsed();

    println!("Lowest possible score: {}", result.0);
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 16 - Part 2");
    let start = std::time::Instant::now();

    let maze = Maze::from_str(INPUT).unwrap();
    let result = maze.find_all_best_paths().unwrap();

    let duration = start.elapsed();

    println!("Lowest possible score: {}", result.1);
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

    const EXAMPLE_1: &str = "\
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

    const EXAMPLE_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_example_part_1() {
        let maze = Maze::from_str(EXAMPLE_1).unwrap();
        let result = maze.find_all_best_paths().unwrap();

        assert_eq!(result.0, 7036);
    }

    #[test]
    fn test_example_part_1_v2() {
        let maze = Maze::from_str(EXAMPLE_2).unwrap();

        let result = maze.find_all_best_paths().unwrap();

        assert_eq!(result.0, 11048);
    }
}
