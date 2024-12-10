///
/// # day_10.rs
/// Code for the day 10 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::{collections::HashSet, str::FromStr};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_10.txt");

#[derive(Debug)]
struct HeightMap {
    heights: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights: Vec<Vec<u32>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let height = heights.len();
        let width = heights[0].len();

        Ok(HeightMap {
            heights,
            width,
            height,
        })
    }
}

impl HeightMap {
    ///
    /// # `find_trailheads`
    /// Find the trailheads starting points.
    ///
    /// ## Returns
    /// * `Vec<(usize, usize)>` - The list of trailheads positions.
    fn find_trailheads(&self) -> Vec<(usize, usize)> {
        let mut trailheads = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.heights[y][x] == 0 {
                    trailheads.push((x, y));
                }
            }
        }
        trailheads
    }

    ///
    /// # `calculate_trailhead_score`
    /// Calculate the score of a trailhead.
    ///
    /// ## Arguments
    /// * `start` - The starting position of the trailhead.
    ///
    /// ## Returns
    /// * `usize` - The score of the trailhead.
    fn calculate_trailhead_score(&self, start: (usize, usize)) -> usize {
        let mut visited = HashSet::new();
        let (_, reachable_nines) = self.traverse_paths(start, &mut visited, 0);

        reachable_nines.len()
    }

    ///
    /// # `calculate_trailhead_rating`
    /// Calculate the rating of a trailhead.
    /// A trailhead's rating is the number of distinct hiking trails which begin at that trailhead.
    ///
    /// ## Arguments
    /// * `start` - The starting position of the trailhead.
    ///
    /// ## Returns
    /// * `usize` - The rating of the trailhead.
    fn calculate_trailhead_rating(&self, start: (usize, usize)) -> usize {
        let mut visited = HashSet::new();
        let (paths, _) = self.traverse_paths(start, &mut visited, 0);

        paths
    }

    ///
    /// # `traverse_paths`
    /// Traverses all possible paths from a starting position, counting valid paths and collecting reachable height-9 positions.
    ///
    /// ## Algorithm: Depth-First Search with Path Tracking
    /// This function implements a modified DFS algorithm that simultaneously:
    /// 1. Counts unique valid paths to height-9 positions
    /// 2. Collects all reachable height-9 positions
    ///
    /// ### Key Characteristics:
    /// - Path Constraints: Must increase exactly by 1 in height at each step
    /// - Movement: Only in 4 directions (up, down, left, right)
    /// - Valid Path: Any path from height 0 to height 9
    ///
    /// ### Implementation Steps:
    /// 1. Base Cases:
    ///    - Return (0, empty_set) if position was already visited
    ///    - Return (0, empty_set) if current height doesn't match expected
    ///    - Return (1, set_with_current) if height is 9
    ///
    /// 2. Recursive Exploration:
    ///    - Mark current position as visited
    ///    - For each adjacent position:
    ///      - Check if it's within bounds
    ///      - Check if its height is exactly current_height + 1
    ///      - Recursively explore valid positions
    ///      - Accumulate paths count and reachable nines
    ///
    /// ## Arguments
    /// * `pos` - The current position (x, y)
    /// * `visited` - Set of already visited positions
    /// * `current_height` - The expected height at the current position
    ///
    /// ## Returns
    /// * `(usize, HashSet<(usize, usize)>)` - A tuple containing:
    ///   - The number of valid paths found
    ///   - Set of all reachable height-9 positions
    ///
    /// ## Details
    /// A valid path must:
    /// - Increase by exactly 1 in height at each step
    /// - Only move in cardinal directions (up, down, left, right)
    /// - Start at height 0 and can reach positions of height 9
    ///
    fn traverse_paths(
        &self,
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
        current_height: u32,
    ) -> (usize, HashSet<(usize, usize)>) {
        // Return early if position was already visited
        if visited.contains(&pos) {
            return (0, HashSet::new());
        }

        let (x, y) = pos;
        let height = self.heights[y][x];

        // Return early if height doesn't match expected height
        if height != current_height {
            return (0, HashSet::new());
        }

        // Mark current position as visited
        visited.insert(pos);

        // If we reached height 9, we found a valid endpoint
        if height == 9 {
            let mut nines = HashSet::new();
            nines.insert(pos);
            return (1, nines);
        }

        // Initialize accumulators for recursive exploration
        let mut total_paths = 0;
        let mut reachable_nines = HashSet::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Explore all adjacent positions
        for (dx, dy) in directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            // Check if new position is within bounds
            if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
                let next_pos = (new_x as usize, new_y as usize);
                let next_height = self.heights[next_pos.1][next_pos.0];

                // Only proceed if height increases by exactly 1
                if next_height == height + 1 {
                    let (paths, nines) =
                        self.traverse_paths(next_pos, &mut visited.clone(), height + 1);
                    total_paths += paths;
                    reachable_nines.extend(nines);
                }
            }
        }

        (total_paths, reachable_nines)
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 10 - Part 1");
    let start = std::time::Instant::now();

    let height_map = INPUT.parse::<HeightMap>().unwrap();
    let trailheads = height_map.find_trailheads();
    let total_score: usize = trailheads
        .iter()
        .map(|&pos| height_map.calculate_trailhead_score(pos))
        .sum();

    let duration = start.elapsed();

    println!("Total score: {total_score}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 10 - Part 2");
    let start = std::time::Instant::now();

    let height_map = INPUT.parse::<HeightMap>().unwrap();
    let trailheads = height_map.find_trailheads();
    let total_rating: usize = trailheads
        .iter()
        .map(|&pos| height_map.calculate_trailhead_rating(pos))
        .sum();

    let duration = start.elapsed();

    println!("Total rating: {total_rating}");
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

    const EXAMPLE1: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_parse_height_map() {
        let height_map = EXAMPLE1.parse::<HeightMap>().unwrap();

        assert_eq!(height_map.width, 8);
        assert_eq!(height_map.height, 8);
        assert_eq!(height_map.heights[0][0], 8);
        assert_eq!(height_map.heights[7][7], 2);
    }

    #[test]
    fn test_example_1() {
        let height_map = EXAMPLE1.parse::<HeightMap>().unwrap();
        let trailheads = height_map.find_trailheads();
        let total_score: usize = trailheads
            .iter()
            .map(|&pos| height_map.calculate_trailhead_score(pos))
            .sum();

        assert_eq!(total_score, 36);
    }

    #[test]
    fn test_example_2() {
        let height_map = EXAMPLE1.parse::<HeightMap>().unwrap();
        let trailheads = height_map.find_trailheads();
        let total_rating: usize = trailheads
            .iter()
            .map(|&pos| height_map.calculate_trailhead_rating(pos))
            .sum();

        assert_eq!(total_rating, 81);
    }
}
