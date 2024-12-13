///
/// # day_12.rs
/// Code for the day 12 of the Advent of Code challenge year 2024
///
/// This code solves a problem involving calculating prices for different regions in a garden plot.
/// The garden is represented as a grid where each character represents a different type of plant.
/// Adjacent cells with the same plant type form regions.
///
/// ## Part 1
/// Calculates region prices based on:
/// - Area (number of cells in the region)
/// - Perimeter (number of edges adjacent to different plant types or garden borders)
///
/// Price = Area × Perimeter
///
/// ## Part 2
/// Uses a different pricing formula based on:
/// - Area (number of cells in the region)
/// - Number of distinct sides (continuous boundaries, counting both outer and inner edges)
///
/// Price = Area × Number of Sides
///
/// ## Implementation Details
/// - Uses flood fill algorithm to identify connected regions
/// - Implements boundary detection for perimeter calculation
/// - Uses HashSets for efficient boundary cell tracking
/// - Handles complex cases including:
///   * Regions with holes
///   * Nested regions
///   * Irregular shapes
///
/// ## Key Components
/// - Garden struct: Represents the garden grid and contains all processing methods
/// - find_regions: Identifies all distinct plant regions
/// - calculate_perimeter: Counts edges for part 1 pricing
/// - calculate_sides: Counts distinct boundaries for part 2 pricing
/// - flood_fill: Recursive algorithm for region detection
///
// Imports  ==============================================================================  Imports
use std::{collections::HashSet, ops::Add, str::FromStr};

use aoc_2024::Direction;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_12.txt");

#[derive(Debug)]
struct Garden {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };

        Ok(Garden {
            grid,
            height,
            width,
        })
    }
}

impl Garden {
    ///
    /// # `find_regions`
    /// Find all regions of the garden.
    /// A region is a group of adjacent cells with the same plant type.
    ///
    /// ## Returns
    /// * `Vec<Vec<(usize, usize)>>` - A vector of regions, where each region is a vector of coordinates of the cells in the region
    fn find_regions(&self) -> Vec<Vec<(usize, usize)>> {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut regions = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if !visited[y][x] {
                    let mut region = Vec::new();
                    self.flood_fill(x, y, self.grid[y][x], &mut visited, &mut region);
                    if !region.is_empty() {
                        regions.push(region);
                    }
                }
            }
        }

        regions
    }

    ///
    /// # `flood_fill`
    /// Recursive function to fill a region of the garden with a plant type.
    ///
    /// ## Arguments
    /// * `x` - The x coordinate of the cell
    /// * `y` - The y coordinate of the cell
    /// * `plant_type` - The type of plant to fill the region with
    /// * `visited` - A 2D vector of booleans to keep track of visited cells
    /// * `region` - A vector of coordinates of the cells in the region
    fn flood_fill(
        &self,
        x: usize,
        y: usize,
        plant_type: char,
        visited: &mut Vec<Vec<bool>>,
        region: &mut Vec<(usize, usize)>, // Now stores (y, x)
    ) {
        if visited[y][x] || self.grid[y][x] != plant_type {
            return;
        }

        visited[y][x] = true;
        region.push((y, x)); // Changed from (x, y) to (y, x)

        // Check all four adjacent cells
        let neighbors = [
            (x, y).add(aoc_2024::Direction::Left),
            (x, y).add(aoc_2024::Direction::Right),
            (x, y).add(aoc_2024::Direction::Down),
            (x, y).add(aoc_2024::Direction::Up),
        ];

        for (nx, ny) in neighbors {
            if nx < self.width && ny < self.height {
                self.flood_fill(nx, ny, plant_type, visited, region);
            }
        }
    }

    ///
    /// # `calculate_perimeter`
    /// Calculate the perimeter of a region.
    /// The perimeter is the number of cells that are adjacent to a cell of a different plant type.
    ///
    /// ## Arguments
    /// * `region` - A vector of coordinates of the cells in the region
    ///
    /// ## Returns
    /// * `u64` - The perimeter of the region
    fn calculate_region_price(&self, region: &[(usize, usize)]) -> u64 {
        let area = region.len() as u64;
        let perimeter = self.calculate_perimeter(region);

        area * perimeter
    }

    ///
    /// # `calculate_perimeter`
    /// Calculate the perimeter of a region.
    /// The perimeter is the number of cells that are adjacent to a cell of a different plant type.
    ///
    /// ## Arguments
    /// * `region` - A vector of coordinates of the cells in the region
    ///
    /// ## Returns
    /// * `u64` - The perimeter of the region
    fn calculate_perimeter(&self, region: &[(usize, usize)]) -> u64 {
        let mut perimeter = 0;
        let region_set: std::collections::HashSet<_> = region.iter().cloned().collect();

        for &(x, y) in region {
            // Check all four sides of the current cell
            let neighbors = [
                (x, y).add(aoc_2024::Direction::Left),  // left
                (x, y).add(aoc_2024::Direction::Right), // right
                (x, y).add(aoc_2024::Direction::Down),  // down
                (x, y).add(aoc_2024::Direction::Up),    // up
            ];

            for (nx, ny) in neighbors {
                // A side contributes to perimeter if it's:
                // 1. On the edge of the grid, or
                // 2. Adjacent to a different plant type
                if nx >= self.width || ny >= self.height || !region_set.contains(&(nx, ny)) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    ///
    /// # `calculate_sides`
    /// Calculate the number of distinct sides of a region.
    /// A side is counted as one unit regardless of its length.
    /// For regions with holes, both inner and outer sides are counted.
    ///
    /// ## Arguments
    /// * `region` - A vector of coordinates of the cells in the region
    ///
    /// ## Returns
    /// * `u64` - The number of distinct sides in the region
    fn calculate_sides(&self, region: &[(usize, usize)]) -> u64 {
        let region_cells: HashSet<(usize, usize)> = region.iter().cloned().collect();
        let boundary_cells = self.find_boundary_cells(region, &region_cells);
        let continuous_boundaries = self.group_boundaries_by_direction(boundary_cells);

        continuous_boundaries.len() as u64
    }

    ///
    /// # `find_boundary_cells`
    /// Finds all cells that form the boundary of a region.
    /// A boundary cell is adjacent to the region but not part of it.
    ///
    /// ## Arguments
    /// * `region` - The cells that make up the region
    /// * `region_cells` - HashSet of the region cells for efficient lookup
    ///
    /// ## Returns
    /// * A HashSet of ((x, y), direction) pairs representing boundary cells and their direction relative to the region
    fn find_boundary_cells(
        &self,
        region: &[(usize, usize)],
        region_cells: &HashSet<(usize, usize)>,
    ) -> HashSet<((usize, usize), Direction)> {
        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        region
            .iter()
            .flat_map(|&cell| {
                directions.iter().filter_map(move |&direction| {
                    let adjacent_cell = cell.add(direction);
                    let (x, y) = adjacent_cell;

                    // Check if the adjacent cell is outside the region
                    if x >= self.width || y >= self.height || !region_cells.contains(&adjacent_cell)
                    {
                        Some((adjacent_cell, direction))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    ///
    /// # `group_boundaries_by_direction`
    /// Groups boundary cells into continuous boundaries based on their direction.
    /// Each continuous boundary represents one "side" of the region.
    ///
    /// ## Arguments
    /// * `boundary_cells` - Set of boundary cells with their directions
    ///
    /// ## Returns
    /// * Vector of HashSets, where each HashSet contains the cells forming one continuous boundary
    fn group_boundaries_by_direction(
        &self,
        boundary_cells: HashSet<((usize, usize), Direction)>,
    ) -> Vec<HashSet<(usize, usize)>> {
        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let mut continuous_boundaries = Vec::new();

        for direction in directions {
            // Get all boundary cells for the current direction
            let mut direction_cells: HashSet<(usize, usize)> = boundary_cells
                .iter()
                .filter(|(_, dir)| *dir == direction)
                .map(|((x, y), _)| (*x, *y))
                .collect();

            // Process cells until we've found all continuous boundaries in this direction
            while !direction_cells.is_empty() {
                if let Some(boundary) = self.extract_continuous_boundary(&mut direction_cells) {
                    continuous_boundaries.push(boundary);
                }
            }
        }

        continuous_boundaries
    }

    ///
    /// # `extract_continuous_boundary`
    /// Extracts a single continuous boundary from a set of cells.
    /// Uses flood-fill algorithm to find all connected cells.
    ///
    /// ## Arguments
    /// * `remaining_cells` - Set of unprocessed boundary cells
    ///
    /// ## Returns
    /// * Option<HashSet<(usize, usize)>> - The extracted continuous boundary, if any
    fn extract_continuous_boundary(
        &self,
        remaining_cells: &mut HashSet<(usize, usize)>,
    ) -> Option<HashSet<(usize, usize)>> {
        let &start_cell = remaining_cells.iter().next()?;
        let mut continuous_boundary = HashSet::new();
        let mut cells_to_check = vec![start_cell];
        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        while let Some(current_cell) = cells_to_check.pop() {
            if continuous_boundary.insert(current_cell) {
                remaining_cells.remove(&current_cell);

                // Check adjacent cells
                for &direction in &directions {
                    let adjacent_cell = current_cell.add(direction);
                    if remaining_cells.contains(&adjacent_cell) {
                        cells_to_check.push(adjacent_cell);
                    }
                }
            }
        }

        Some(continuous_boundary)
    }

    ///
    /// # `calculate_region_price_part2`
    /// Calculate the price of a region for part 2.
    ///
    /// ## Arguments
    /// * `region` - A vector of coordinates of the cells in the region
    ///
    /// ## Returns
    /// * `u64` - The price of the region
    fn calculate_region_price_part_2(&self, region: &[(usize, usize)]) -> u64 {
        let area = region.len() as u64;
        let sides = self.calculate_sides(region);

        area * sides
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 12 - Part 1");
    let start = std::time::Instant::now();

    let garden: Garden = INPUT.parse().unwrap();
    let regions = garden.find_regions();
    let total_price: u64 = regions
        .iter()
        .map(|region| garden.calculate_region_price(region))
        .sum();

    let duration = start.elapsed();

    println!("Total price: {}", total_price);
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 12 - Part 2");
    let start = std::time::Instant::now();

    let garden: Garden = INPUT.parse().unwrap();
    let regions = garden.find_regions();
    let total_price: u64 = regions
        .iter()
        .map(|region| garden.calculate_region_price_part_2(region))
        .sum();

    let duration = start.elapsed();

    println!("Total price: {}", total_price);
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

    const SIMPLE_EXAMPLE: &str = "\
AAAA
BBCD
BBCC
EEEC";

    const XOXO_EXAMPLE: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE: &str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const LARGE_EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_from_str() {
        let garden = Garden::from_str(EXAMPLE).unwrap();

        assert_eq!(garden.height, 10);
        assert_eq!(garden.width, 10);
    }

    #[test]
    fn test_simple_garden() {
        let garden: Garden = SIMPLE_EXAMPLE.parse().unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price(region))
            .sum();

        assert_eq!(total_price, 140);
    }

    #[test]
    fn test_nested_regions() {
        let garden = Garden::from_str(XOXO_EXAMPLE).unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price(region))
            .sum();

        assert_eq!(total_price, 772);
    }

    #[test]
    fn test_larger_garden() {
        let garden = Garden::from_str(LARGE_EXAMPLE).unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price(region))
            .sum();

        assert_eq!(total_price, 1930);
    }

    #[test]
    fn test_calculate_sides() {
        let garden = Garden::from_str(SIMPLE_EXAMPLE).unwrap();
        let regions = garden.find_regions();

        println!("{:?}", regions);

        let a_sides = garden.calculate_sides(&regions[0]);
        let b_sides = garden.calculate_sides(&regions[1]);
        let c_sides = garden.calculate_sides(&regions[2]);
        let d_sides = garden.calculate_sides(&regions[3]);
        let e_sides = garden.calculate_sides(&regions[4]);

        assert_eq!(a_sides, 4);
        assert_eq!(b_sides, 4);
        assert_eq!(c_sides, 8);
        assert_eq!(d_sides, 4);
        assert_eq!(e_sides, 4);
    }

    #[test]
    fn test_simple_garden_part_2() {
        let garden: Garden = SIMPLE_EXAMPLE.parse().unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price_part_2(region))
            .sum();

        assert_eq!(total_price, 80);
    }

    #[test]
    fn test_nested_regions_part_2() {
        let garden = Garden::from_str(XOXO_EXAMPLE).unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price_part_2(region))
            .sum();

        assert_eq!(total_price, 436);
    }

    #[test]
    fn test_part_2_ex_1() {
        let input = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

        let garden = Garden::from_str(input).unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price_part_2(region))
            .sum();

        assert_eq!(total_price, 236);
    }

    #[test]
    fn test_part_2_ex_2() {
        let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        let garden = Garden::from_str(input).unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price_part_2(region))
            .sum();

        assert_eq!(total_price, 368);
    }

    #[test]
    fn test_larger_garden_part_2() {
        let garden = Garden::from_str(LARGE_EXAMPLE).unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price_part_2(region))
            .sum();

        assert_eq!(total_price, 1206);
    }
}
