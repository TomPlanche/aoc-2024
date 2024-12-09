///
/// # `day_04.rs`
/// Code for the day 04 of the Advent of Code challenge year 2024
///
/// Algorithm Description:
///
/// Part 1: Search for "XMAS" Occurrences
/// - Parse input into a grid structure
/// - Search for the word "XMAS" in all 8 directions (horizontal, vertical, and diagonal)
/// - For each position in the grid:
///   - Check each direction using delta coordinates
///   - Verify if all characters match "XMAS" sequentially
/// - Count total valid occurrences
///
/// Part 2: Find X-MAS Patterns
/// - Look for specific cross patterns where:
///   - 'A' is in the center
///   - 'X', 'M', 'S' form an X-pattern around the 'A'
///   - Diagonal pairs must sum to specific ASCII values (160)
/// - Count total valid X-patterns in the grid
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_04.txt");
const TARGET: &str = "XMAS";

// Structs ============================================================================== Structs
#[derive(Debug)]
struct Grid {
    data: Vec<String>,
    height: usize,
    width: usize,
}

#[derive(Debug, Copy, Clone)]
struct Direction {
    row_delta: i32,
    col_delta: i32,
}

// Implementation ======================================================================= Implementation
impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<String> = s.lines().map(String::from).collect();
        if data.is_empty() {
            return Err(());
        }

        let height = data.len();
        let width = data[0].len();

        Ok(Grid {
            data,
            height,
            width,
        })
    }
}

impl Grid {
    fn get_char(&self, row: i32, col: i32) -> Option<char> {
        if row >= 0
            && col >= 0
            && row < i32::try_from(self.height).unwrap()
            && col < i32::try_from(self.width).unwrap()
        {
            self.data[row as usize].chars().nth(col as usize)
        } else {
            None
        }
    }

    fn check_word_at_position(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
        target: &str,
    ) -> bool {
        target.chars().enumerate().all(|(i, target_char)| {
            let new_row =
                i32::try_from(row).unwrap() + direction.row_delta * i32::try_from(i).unwrap();
            let new_col =
                i32::try_from(col).unwrap() + direction.col_delta * i32::try_from(i).unwrap();

            self.get_char(new_row, new_col)
                .map_or(false, |c| c == target_char)
        })
    }

    fn count_xmas_occurrences(&self) -> usize {
        let directions = [
            Direction {
                row_delta: 0,
                col_delta: 1,
            }, // right
            Direction {
                row_delta: 1,
                col_delta: 0,
            }, // down
            Direction {
                row_delta: 1,
                col_delta: 1,
            }, // diagonal down-right
            Direction {
                row_delta: -1,
                col_delta: 1,
            }, // diagonal up-right
            Direction {
                row_delta: 0,
                col_delta: -1,
            }, // left
            Direction {
                row_delta: -1,
                col_delta: 0,
            }, // up
            Direction {
                row_delta: -1,
                col_delta: -1,
            }, // diagonal up-left
            Direction {
                row_delta: 1,
                col_delta: -1,
            }, // diagonal down-left
        ];

        let mut count = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                for &direction in &directions {
                    if self.check_word_at_position(row, col, direction, TARGET) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn count_x_mas_patterns(&self) -> usize {
        let mut count = 0;

        for row in 1..self.height - 1 {
            for col in 1..self.width - 1 {
                if self.get_char(i32::try_from(row).unwrap(), i32::try_from(col).unwrap())
                    != Some('A')
                {
                    continue;
                }

                let corners = [
                    (row - 1, col - 1), // top-left
                    (row - 1, col + 1), // top-right
                    (row + 1, col - 1), // bottom-left
                    (row + 1, col + 1), // bottom-right
                ];

                let chars: Vec<u32> = corners
                    .iter()
                    .filter_map(|&(r, c)| {
                        self.get_char(i32::try_from(r).unwrap(), i32::try_from(c).unwrap())
                            .map(u32::from)
                    })
                    .collect();

                if chars.len() == 4
                    && chars[0] + chars[3] == 160 // diagonal pair 1
                    && chars[1] + chars[2] == 160
                {
                    count += 1;
                }
            }
        }
        count
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 04 - Part 1");

    let start = std::time::Instant::now();

    let grid: Grid = INPUT.parse().unwrap();
    let result = grid.count_xmas_occurrences();

    let duration = start.elapsed();

    println!("Number of XMAS occurrences: {result}");
    println!("Duration: {duration:?}\n");
}

pub fn response_part_2() {
    println!("Day 04 - Part 2");

    let start = std::time::Instant::now();

    let grid: Grid = INPUT.parse().unwrap();
    let result = grid.count_x_mas_patterns();

    let duration = start.elapsed();

    println!("Number of X-MAS patterns: {result}");
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

    fn create_test_grid() -> Grid {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM";
        input.parse().unwrap()
    }

    #[test]
    fn test_grid_creation() {
        let grid = create_test_grid();
        assert_eq!(grid.height, 5);
        assert_eq!(grid.width, 10);
    }

    #[test]
    fn test_get_char() {
        let grid = create_test_grid();
        assert_eq!(grid.get_char(0, 0), Some('M'));
        assert_eq!(grid.get_char(-1, 0), None);
        assert_eq!(grid.get_char(0, 10), None);
    }

    #[test]
    fn test_xmas_occurrences() {
        let grid = create_test_grid();
        assert!(grid.count_xmas_occurrences() > 0);
    }

    #[test]
    fn test_x_mas_patterns() {
        let input = "\
M.S
.A.
M.S";
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.count_x_mas_patterns(), 1);
    }
}
