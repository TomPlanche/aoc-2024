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
        let mut reachable_nines = HashSet::new();

        self.dfs(start, &mut visited, &mut reachable_nines, 0);

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
        self.count_paths(start, &mut visited, 0)
    }

    ///
    /// # `count_paths`
    /// Count the number of paths that can be taken from a trailhead.
    ///
    /// ## Arguments
    /// * `pos` - The current position.
    /// * `visited` - The set of visited positions.
    /// * `current_height` - The current height.
    ///
    /// ## Returns
    /// * `usize` - The number of paths that can be taken.
    fn count_paths(
        &self,
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
        current_height: u32,
    ) -> usize {
        if visited.contains(&pos) {
            return 0;
        }

        let (x, y) = pos;
        let height = self.heights[y][x];

        if height != current_height {
            return 0;
        }

        if height == 9 {
            return 1; // Found a valid path
        }

        visited.insert(pos);

        let mut paths = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dx, dy) in directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
                let next_pos = (new_x as usize, new_y as usize);
                let next_height = self.heights[next_pos.1][next_pos.0];

                if next_height == height + 1 {
                    paths += self.count_paths(next_pos, &mut visited.clone(), height + 1);
                }
            }
        }

        paths
    }

    ///
    /// # `dfs`
    /// Depth First Search to find all the reachable nines.
    ///
    /// ## Arguments
    /// * `pos` - The current position.
    /// * `visited` - The set of visited positions.
    /// * `reachable_nines` - The set of reachable nines.
    /// * `current_height` - The current height.
    ///
    /// ## Explanation
    /// This function performs a depth-first search to find all height-9 positions that can be reached
    /// from a trailhead following valid hiking trails. A valid trail:
    /// - Must increase by exactly 1 in height at each step
    /// - Can only move up, down, left, or right (no diagonals)
    /// - Starts at height 0 and can reach positions of height 9
    ///
    /// The function:
    /// 1. Checks if position was already visited to avoid cycles
    /// 2. Validates the current height matches expected height
    /// 3. Marks position as visited
    /// 4. If height is 9, adds position to reachable_nines
    /// 5. Recursively explores adjacent positions that are exactly 1 height
    fn dfs(
        &self,
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
        reachable_nines: &mut HashSet<(usize, usize)>,
        current_height: u32,
    ) {
        if visited.contains(&pos) {
            return;
        }

        let (x, y) = pos;
        let height = self.heights[y][x];

        if height != current_height {
            return;
        }

        visited.insert(pos);

        if height == 9 {
            reachable_nines.insert(pos);
            return;
        }

        // Check all adjacent positions
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
                let next_pos = (new_x as usize, new_y as usize);
                let next_height = self.heights[next_pos.1][next_pos.0];

                if next_height == height + 1 {
                    self.dfs(next_pos, visited, reachable_nines, height + 1);
                }
            }
        }
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
