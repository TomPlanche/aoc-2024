///
/// # Day 4: Ceres Search
/// Solution for the Advent of Code 2024 - Day 4
/// Finds all occurrences of "XMAS" in a word search grid, including diagonal,
/// backwards, and overlapping instances.
///

// Constants =========================================================================== Constants
const INPUT: &str = include_str!("../../data/inputs/day_04.txt");
const TARGET: &str = "XMAS";

// Functions =========================================================================== Functions
///
/// # count_xmas_occurrences
///
/// Counts the number of times "XMAS" appears in the word search grid.
/// Searches in all eight directions: horizontal, vertical, and diagonal.
///
/// ## Arguments
///
/// * `grid` - A vector of string slices representing the word search grid
///
/// ## Returns
///
/// * `usize` - The total number of "XMAS" occurrences found
///
fn count_xmas_occurrences(grid: &[&str]) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // Direction vectors for all 8 possible directions
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (-1, 1),  // diagonal up-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
    ];

    for i in 0..height {
        for j in 0..width {
            for &(di, dj) in directions.iter() {
                if check_word_at_position(grid, i, j, di, dj, TARGET) {
                    count += 1;
                }
            }
        }
    }

    count
}

///
/// # check_word_at_position
///
/// Checks if the target word exists starting from a given position in a specific direction.
///
/// ## Arguments
///
/// * `grid` - The word search grid
/// * `row` - Starting row position
/// * `col` - Starting column position
/// * `di` - Row direction (-1, 0, or 1)
/// * `dj` - Column direction (-1, 0, or 1)
/// * `target` - The word to search for
///
/// ## Returns
///
/// * `bool` - True if the word is found, false otherwise
///
fn check_word_at_position(
    grid: &[&str],
    row: usize,
    col: usize,
    di: i32,
    dj: i32,
    target: &str,
) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let chars: Vec<char> = target.chars().collect();

    for (idx, &target_char) in chars.iter().enumerate() {
        let new_row = row as i32 + di * idx as i32;
        let new_col = col as i32 + dj * idx as i32;

        if new_row < 0 || new_row >= height || new_col < 0 || new_col >= width {
            return false;
        }

        if grid[new_row as usize]
            .chars()
            .nth(new_col as usize)
            .unwrap()
            != target_char
        {
            return false;
        }
    }
    true
}

///
/// # count_x_mas_occurrences
///
/// Counts the number of X-shaped MAS patterns in the grid.
/// Each arm of the X can be either "MAS" or "SAM" (forwards or backwards).
///
/// ## Arguments
///
/// * `grid` - A vector of string slices representing the word search grid
///
/// ## Returns
///
/// * `usize` - The total number of X-MAS patterns found
///
fn count_x_mas_occurrences(grid: &[&str]) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // We need at least 3x3 space to form an X-MAS
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if check_x_mas_at_position(grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

///
/// # check_x_mas_at_position
///
/// Checks if an X-MAS pattern exists with its center at the given position.
/// The pattern consists of two "MAS" strings forming an X shape.
///
/// ## Arguments
///
/// * `grid` - The word search grid
/// * `row` - Center row position
/// * `col` - Center column position
///
/// ## Returns
///
/// * `bool` - True if an X-MAS pattern is found, false otherwise
///
fn check_x_mas_at_position(grid: &[&str], row: usize, col: usize) -> bool {
    let directions = [
        // Down-right and up-left combination
        ((1, 1), (-1, -1)),
        // Up-right and down-left combination
        ((-1, 1), (1, -1)),
    ];

    for &(dir1, dir2) in &directions {
        if check_x_mas_arms(grid, row, col, dir1, dir2) {
            return true;
        }
    }
    false
}

///
/// # check_x_mas_arms
///
/// Checks if two valid MAS strings exist in the specified directions forming an X.
///
/// ## Arguments
///
/// * `grid` - The word search grid
/// * `row` - Center row position
/// * `col` - Center column position
/// * `dir1` - Direction tuple for first arm (dy, dx)
/// * `dir2` - Direction tuple for second arm (dy, dx)
///
/// ## Returns
///
/// * `bool` - True if both arms form valid MAS strings
///
fn check_x_mas_arms(
    grid: &[&str],
    row: usize,
    col: usize,
    dir1: (i32, i32),
    dir2: (i32, i32),
) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    // Helper closure to get character at position
    let get_char = |r: i32, c: i32| -> Option<char> {
        if r >= 0 && r < height && c >= 0 && c < width {
            Some(grid[r as usize].chars().nth(c as usize).unwrap())
        } else {
            None
        }
    };

    // Extract characters for both arms
    let mut arm1 = Vec::new();
    let mut arm2 = Vec::new();

    for i in -1..=1 {
        let (r1, c1) = (row as i32 + dir1.0 * i, col as i32 + dir1.1 * i);
        let (r2, c2) = (row as i32 + dir2.0 * i, col as i32 + dir2.1 * i);

        if let (Some(ch1), Some(ch2)) = (get_char(r1, c1), get_char(r2, c2)) {
            arm1.push(ch1);
            arm2.push(ch2);
        } else {
            return false;
        }
    }

    // Check if both arms are valid MAS strings (forward or backward)
    let is_valid_mas = |chars: &[char]| -> bool {
        let s: String = chars.iter().collect();
        s == "MAS" || s == "SAM"
    };

    is_valid_mas(&arm1) && is_valid_mas(&arm2)
}

///
/// # parse_input
///
/// Parses the input string into a vector of string slices.
///
/// ## Arguments
///
/// * `input` - The raw input string
///
/// ## Returns
///
/// * `Vec<&str>` - Vector of string slices representing the grid
///
fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn response_part_1() {
    let grid = parse_input(INPUT);
    let result = count_xmas_occurrences(&grid);
    println!("Day 04 - Part 1: {}", result);
}

pub fn response_part_2() {
    let grid = parse_input(INPUT);
    let result = count_x_mas_occurrences(&grid);
    println!("Day 04 - Part 2: {}", result);
}

fn main() {
    response_part_1();
    response_part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas_occurrences() {
        let test_input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        assert_eq!(count_xmas_occurrences(&test_input), 18);
    }

    #[test]
    fn test_check_word_at_position() {
        let grid = vec!["XMAS", "AMXX", "SSAA"];
        assert!(check_word_at_position(&grid, 0, 0, 0, 1, "XMAS")); // horizontal
        assert!(!check_word_at_position(&grid, 0, 0, 1, 0, "XMAS")); // vertical (should fail)
    }
}
