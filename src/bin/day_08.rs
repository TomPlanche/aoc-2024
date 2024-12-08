///
/// # day_08.rs
/// Code for the day 08 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use aoc_2024::{gcd, Point};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_08.txt");

type MyPoint = Point<i32>;

#[derive(Debug, Clone)]
struct Antenna {
    x: i32,
    y: i32,
    frequency: char,
}

#[derive(Debug)]
struct AntennaMap {
    antennas: Vec<Antenna>,
    width: i32,
    height: i32,
}

impl FromStr for AntennaMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas = Vec::new();
        let mut height = 0;
        let mut width = 0;

        for (y, line) in s.lines().enumerate() {
            height = height.max(y + 1);
            width = width.max(line.len());

            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.push(Antenna {
                        x: x as i32,
                        y: y as i32,
                        frequency: c,
                    });
                }
            }
        }

        Ok(AntennaMap {
            antennas,
            width: width as i32,
            height: height as i32,
        })
    }
}

impl AntennaMap {
    ///
    /// # `find_antinodes`
    /// Find antinodes (points that are twice the distance between two antennas)
    ///
    /// ## Returns
    /// * `HashSet<MyPoint>` - The set of antinodes
    fn find_antinodes(&self) -> HashSet<MyPoint> {
        let mut antinodes = HashSet::new();
        let mut freq_map: HashMap<char, Vec<&Antenna>> = HashMap::new();

        // Group antennas by frequency
        for antenna in &self.antennas {
            freq_map.entry(antenna.frequency).or_default().push(antenna);
        }

        // For each frequency group
        for antennas in freq_map.values() {
            // For each pair of antennas with the same frequency
            for i in 0..antennas.len() {
                for j in i + 1..antennas.len() {
                    let a1 = antennas[i];
                    let a2 = antennas[j];

                    // Try both antennas as the reference MyPoint
                    self.add_antinodes(&mut antinodes, a1, a2);
                    self.add_antinodes(&mut antinodes, a2, a1);
                }
            }
        }

        antinodes
    }

    ///
    /// # `add_antinodes`
    /// Add antinodes to the set if they're within bounds
    /// Antinodes are the points that are twice the distance between two antennas
    ///
    /// ## Arguments
    /// * `antinodes` - The set of antinodes
    /// * `a1` - The first antenna
    /// * `a2` - The second antenna
    fn add_antinodes(&self, antinodes: &mut HashSet<MyPoint>, a1: &Antenna, a2: &Antenna) {
        // Vector from a1 to a2
        let dx = a2.x - a1.x;
        let dy = a2.y - a1.y;

        // Calculate potential antinode positions (twice the distance)
        let antinode1 = MyPoint {
            x: a2.x + dx,
            y: a2.y + dy,
        };

        let antinode2 = MyPoint {
            x: a1.x - dx,
            y: a1.y - dy,
        };

        // Add antinodes if they're within bounds
        if self.is_within_bounds(&antinode1) {
            antinodes.insert(antinode1);
        }
        if self.is_within_bounds(&antinode2) {
            antinodes.insert(antinode2);
        }
    }

    ///
    /// # `is_within_bounds`
    /// Check if a point is within the bounds of the map
    ///
    /// ## Arguments
    /// * `my_point` - The point to check
    ///
    /// ## Returns
    /// * `bool` - True if the point is within the bounds
    fn is_within_bounds(&self, my_point: &MyPoint) -> bool {
        my_point.x >= 0 && my_point.x < self.width && my_point.y >= 0 && my_point.y < self.height
    }

    ///
    /// # `find_antinodes_with_harmonics`
    /// Find antinodes with harmonics (antinodes that are multiples of the distance between two antennas)
    ///
    /// ## Returns
    /// * `HashSet<MyPoint>` - The set of antinodes
    fn find_antinodes_with_harmonics(&self) -> HashSet<MyPoint> {
        let mut antinodes = HashSet::new();
        let mut freq_map: HashMap<char, Vec<&Antenna>> = HashMap::new();

        // Group antennas by frequency
        for antenna in &self.antennas {
            freq_map.entry(antenna.frequency).or_default().push(antenna);
        }

        // For each frequency group
        for antennas in freq_map.values() {
            // Skip if there's only one antenna of this frequency
            if antennas.len() < 2 {
                continue;
            }

            // Check all possible pairs of antennas
            for i in 0..antennas.len() {
                for j in i + 1..antennas.len() {
                    let a1 = antennas[i];
                    let a2 = antennas[j];

                    // Vector from a1 to a2
                    let dx = a2.x - a1.x;
                    let dy = a2.y - a1.y;

                    // Find the GCD to get the smallest step size
                    let gcd = gcd(dx.abs(), dy.abs());
                    let step_x = if gcd != 0 { dx / gcd } else { 0 };
                    let step_y = if gcd != 0 { dy / gcd } else { 0 };

                    // Add points along the line in both directions
                    let mut current = MyPoint { x: a1.x, y: a1.y };

                    // Forward direction
                    while self.is_within_bounds(&current) {
                        antinodes.insert(current);
                        current.x += step_x;
                        current.y += step_y;
                    }

                    // Backward direction
                    let mut current = MyPoint {
                        x: a1.x - step_x,
                        y: a1.y - step_y,
                    };
                    while self.is_within_bounds(&current) {
                        antinodes.insert(current);
                        current.x -= step_x;
                        current.y -= step_y;
                    }
                }
            }
        }

        antinodes
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 08 - Part 1");
    let start = std::time::Instant::now();

    let map: AntennaMap = INPUT.parse().unwrap();
    let antinodes = map.find_antinodes();
    let count = antinodes.len();

    let duration = start.elapsed();

    println!("Found {count} antinodes");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 08 - Part 2");
    let start = std::time::Instant::now();

    let map: AntennaMap = INPUT.parse().unwrap();
    let antinodes = map.find_antinodes_with_harmonics();
    let count = antinodes.len();

    let duration = start.elapsed();
    println!("Found {count} antinodes with harmonics");
    println!("Time elapsed: {duration:?}");
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse_map() {
        let map: AntennaMap = TEST_INPUT.parse().unwrap();
        assert_eq!(map.width, 12);
        assert_eq!(map.height, 12);
        assert_eq!(map.antennas.len(), 7); // 4 zeros and 3 A's
    }

    #[test]
    fn test_find_antinodes() {
        let map: AntennaMap = TEST_INPUT.parse().unwrap();
        let antinodes = map.find_antinodes();
        assert_eq!(antinodes.len(), 14);
    }

    #[test]
    fn test_find_antinodes_with_harmonics() {
        let map: AntennaMap = TEST_INPUT.parse().unwrap();
        let antinodes = map.find_antinodes_with_harmonics();
        assert_eq!(antinodes.len(), 34);
    }
}
