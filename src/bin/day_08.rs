///
/// # day_08.rs
/// Code for the day 08 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use aoc_2024::Point;
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

                    // Calculate distance between antennas
                    let dx = a2.x - a1.x;
                    let dy = a2.y - a1.y;
                    let dist = (dx * dx + dy * dy) as f64;

                    // Try both antennas as the reference MyPoint
                    self.add_antinodes(&mut antinodes, a1, a2);
                    self.add_antinodes(&mut antinodes, a2, a1);
                }
            }
        }

        antinodes
    }

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

    fn is_within_bounds(&self, MyPoint: &MyPoint) -> bool {
        MyPoint.x >= 0 && MyPoint.x < self.width && MyPoint.y >= 0 && MyPoint.y < self.height
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

    println!("Found {} antinodes", count);
    println!("Duration: {:?}", duration);
}

pub fn response_part_2() {
    println!("Day 08 - Part 2");
    let start = std::time::Instant::now();

    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
    println!("Duration: {:?}", duration);
}

fn main() {
    response_part_1();
    response_part_2();
}
// Tests ==================================================================================== Tests
