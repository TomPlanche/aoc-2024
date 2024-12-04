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
/// A valid X-MAS pattern consists of:
/// - An 'A' in the center
/// - Two diagonal pairs of 'M' and 'S' that form an X
/// - Each diagonal must have one 'M' and one 'S' (in any order)
///
/// The solution uses the ASCII property that 'M' (77) + 'S' (83) = 160
/// This lets us verify each diagonal pair with a single sum check.
///
/// Valid patterns:
/// ```text
/// M.S     S.S
/// .A.  or .A.
/// M.S     M.M
/// ```
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
    let mut count = 0;

    // Iterate through all possible center positions (excluding edges)
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            // Check if center is 'A'
            if grid[i].chars().nth(j).unwrap() == 'A' {
                // Get the four corner characters of the X pattern
                let top_left = grid[i - 1].chars().nth(j - 1).unwrap() as u32; // Can be 'M' or 'S'
                let bottom_right = grid[i + 1].chars().nth(j + 1).unwrap() as u32; // Must pair with top_left
                let bottom_left = grid[i + 1].chars().nth(j - 1).unwrap() as u32; // Can be 'M' or 'S'
                let top_right = grid[i - 1].chars().nth(j + 1).unwrap() as u32; // Must pair with bottom_left

                // Check if both diagonals are valid M-S pairs (sum = 160)
                if (top_left + bottom_right == 160) && (bottom_left + top_right == 160) {
                    count += 1;
                }
            }
        }
    }

    count
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

    #[test]
    fn test_x_mas_pattern() {
        let test_input = vec!["M.S", ".A.", "M.S"];
        assert_eq!(count_x_mas_occurrences(&test_input), 1);

        let test_input2 = vec!["S.S", ".A.", "M.M"];
        assert_eq!(count_x_mas_occurrences(&test_input2), 1);
    }

    #[test]
    fn test_x_mas_pattern_invalid() {
        let test_input = vec!["M.M", ".A.", "S.S"];
        assert_eq!(count_x_mas_occurrences(&test_input), 0);
    }
}
