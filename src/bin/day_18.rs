///
/// # day_18.rs
/// Code for the day 18 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use aoc_2024::{Direction, Point};
use regex::Regex;
use std::{collections::VecDeque, fmt};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_18.txt");

type MyPoint = Point<usize>;

/// Each byte position is given as an X,Y coordinate,
/// where X is the distance from the left edge of your memory space
/// and Y is the distance from the top edge of your memory space.
#[derive(Debug, Clone, Copy)]
struct Byte {
    coords: MyPoint,
    is_corrupted: bool,
}

struct Grid {
    cells: Vec<Vec<Byte>>, // whether the cell is corrupted
    size: usize,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for byte in row {
                write!(f, "{}", if byte.is_corrupted { "#" } else { "." })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for byte in row {
                write!(f, "{}", if byte.is_corrupted { "#" } else { "." })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    ///
    /// # `new`
    /// Create a new Grid from a string input.
    ///
    /// ## Arguments
    /// * `input` - A string slice that holds the input data.
    /// * `is_test` - A boolean that indicates if the input is a test input.
    ///
    /// ## Returns
    /// * `Self` - A Grid instance.
    fn new(input: &str, is_test: bool) -> Self {
        let size = if is_test { 7 } else { 71 };
        let mut all_cords = Vec::new();

        let coords_regex = Regex::new(r"(?P<number_1>\d+),(?P<number_2>\d+)").unwrap();

        for line in input.trim().lines() {
            for cap in coords_regex.captures_iter(line) {
                let y = cap["number_1"].parse().unwrap();
                let x = cap["number_2"].parse().unwrap();

                all_cords.push(Byte {
                    coords: Point { x, y },
                    is_corrupted: true,
                });
            }
        }

        let mut cells = vec![
            vec![
                Byte {
                    coords: Point { x: 0, y: 0 },
                    is_corrupted: false
                };
                size
            ];
            size
        ];

        for byte in all_cords {
            cells[byte.coords.x][byte.coords.y] = byte;
        }

        Grid { cells, size }
    }

    ///
    /// # `find_shortest_path`
    /// Find the shortest path from a start point to an end point.
    ///
    /// ## Algorithm
    /// Used a Breadth-First Search (BFS) algorithm to find the shortest path.
    ///
    /// ## Arguments
    /// * `start` - The starting point.
    /// * `end` - The ending point.
    ///
    /// ## Returns
    /// * `Option<(usize, Vec<MyPoint>)>` - A tuple containing the number of steps and the path.
    fn find_shortest_path(&self, start: MyPoint, end: MyPoint) -> Option<(usize, Vec<MyPoint>)> {
        let mut visited = vec![vec![false; self.size]; self.size];
        let mut queue = VecDeque::new();
        let mut distances = vec![vec![usize::MAX; self.size]; self.size];
        let mut prev = vec![vec![None; self.size]; self.size]; // Keep track of previous points

        // Start position
        queue.push_back(start);
        visited[start.x][start.y] = true;
        distances[start.x][start.y] = 0;

        // Possible moves: up, down, left, right
        let moves = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct path
                let mut path = Vec::new();
                let mut curr = current;

                path.push(curr);

                while let Some(previous) = prev[curr.x][curr.y] {
                    path.push(previous);
                    curr = previous;
                }

                path.reverse();

                return Some((distances[current.x][current.y], path));
            }

            for direction in moves.iter() {
                let dx = direction.col_delta();
                let dy = direction.row_delta();

                if let (Some(new_x), Some(new_y)) = (
                    current.x.checked_add_signed(dx),
                    current.y.checked_add_signed(dy),
                ) {
                    if new_x < self.size
                        && new_y < self.size
                        && !visited[new_x][new_y]
                        && !self.cells[new_x][new_y].is_corrupted
                    {
                        visited[new_x][new_y] = true;
                        distances[new_x][new_y] = distances[current.x][current.y] + 1;
                        prev[new_x][new_y] = Some(current); // Store the previous point
                        queue.push_back(Point { x: new_x, y: new_y });
                    }
                }
            }
        }

        None
    }

    ///
    /// # `display_with_path`
    /// Display the grid with the path marked.
    ///
    /// ## Arguments
    /// * `path` - A vector of points that represent the path.
    ///
    /// ## Returns
    /// * `String` - A string that represents the grid with the path marked.
    #[allow(dead_code)]
    fn display_with_path(&self, path: &[MyPoint]) -> String {
        let mut output = String::new();
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                let current_point = Point { x, y };
                if path.contains(&current_point) {
                    output.push('O'); // Path marker
                } else if self.cells[x][y].is_corrupted {
                    output.push('#'); // Wall
                } else {
                    output.push('.'); // Empty space
                }
            }
            output.push('\n');
        }
        output
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 18 - Part 1");
    let start = std::time::Instant::now();

    let first_1024_bytes = INPUT
        .trim()
        .lines()
        .take(1024)
        .collect::<Vec<&str>>()
        .join("\n");

    let grid = Grid::new(&first_1024_bytes, false);
    // println!("{}", grid);

    let shortest_path = grid.find_shortest_path(
        Point { x: 0, y: 0 },
        Point {
            x: grid.size - 1,
            y: grid.size - 1,
        },
    );

    if let Some((steps, _path)) = shortest_path {
        println!("Shortest path: {steps}");

        // // Display the path
        // println!("{}", grid.display_with_path(&_path));
    } else {
        println!("No path found");
    }

    let duration = start.elapsed();
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 18 - Part 2");
    let start = std::time::Instant::now();

    let duration = start.elapsed();

    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    // response_part_2();
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
    5,4
    4,2
    4,5
    3,0
    2,1
    6,3
    2,4
    1,5
    0,6
    3,3
    2,6
    5,1
    1,2
    5,5
    2,5
    6,5
    1,4
    0,4
    6,4
    1,1
    6,1
    1,0
    0,5
    1,6
    2,0";

    #[test]
    fn test_grid_from_str() {
        let first_12_bytes = TEST_INPUT
            .lines()
            .take(12)
            .collect::<Vec<&str>>()
            .join("\n");

        let grid = Grid::new(&first_12_bytes, true);

        assert_eq!(
            "\
...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....
", // '\n' is added at the end of the string
            grid.to_string()
        );
    }

    #[test]
    fn test_shortest_path() {
        let first_12_bytes = TEST_INPUT
            .lines()
            .take(12)
            .collect::<Vec<&str>>()
            .join("\n");

        let grid = Grid::new(&first_12_bytes, true);

        let shortest_path = grid.find_shortest_path(Point { x: 0, y: 0 }, Point { x: 6, y: 6 });
        let (steps, path) = shortest_path.unwrap();

        println!("{}", grid.display_with_path(&path));

        assert_eq!(steps, 22);
        assert_eq!(
            "\
OO.#OOO
.O#OO#O
.OOO#OO
...#OO#
..#OO#.
.#.O#..
#.#OOOO
",
            grid.display_with_path(&path)
        );
    }
}
