///
/// # day_12.rs
/// Code for the day 12 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

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
        region: &mut Vec<(usize, usize)>,
    ) {
        if visited[y][x] || self.grid[y][x] != plant_type {
            return;
        }

        visited[y][x] = true;
        region.push((x, y));

        // Check all four adjacent cells
        let neighbors = [
            (x.wrapping_sub(1), y), // left
            (x + 1, y),             // right
            (x, y.wrapping_sub(1)), // up
            (x, y + 1),             // down
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
                (x.wrapping_sub(1), y), // left
                (x + 1, y),             // right
                (x, y.wrapping_sub(1)), // up
                (x, y + 1),             // down
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

    #[test]
    fn test_from_str() {
        let garden = Garden::from_str(EXAMPLE).unwrap();

        assert_eq!(garden.height, 10);
        assert_eq!(garden.width, 10);
    }

    #[test]
    fn test_simple_garden() {
        let input = "\
AAAA
BBCD
BBCC
EEEC";
        let garden: Garden = input.parse().unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price(region))
            .sum();

        assert_eq!(total_price, 140);
    }

    #[test]
    fn test_nested_regions() {
        let input = "\
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO";
        let garden = Garden::from_str(input).unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price(region))
            .sum();

        assert_eq!(total_price, 436);
    }

    #[test]
    fn test_larger_garden() {
        let input = "\
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

        let garden = Garden::from_str(input).unwrap();
        let regions = garden.find_regions();
        let total_price: u64 = regions
            .iter()
            .map(|region| garden.calculate_region_price(region))
            .sum();

        assert_eq!(total_price, 1930);
    }
}
