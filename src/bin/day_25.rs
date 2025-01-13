///
/// # Day 25: Lock and Key Matching (Advent of Code 2024)
///
/// This module solves a problem involving virtual five-pin tumbler locks and keys.
/// Each lock and key is represented as a schematic of '#' and '.' characters,
/// where the height of columns represents pin heights for locks (extending down from top)
/// or key shapes (extending up from bottom).
///
// Imports  ==============================================================================  Imports
use itertools::{Either, Itertools};

// Constants  =========================================================================  Constants
/// Input file containing lock and key schematics
const INPUT: &str = include_str!("../../data/inputs/day_25.txt");

/// Maximum height of the schematic grid (used for overlap checking)
const GRID_HEIGHT: u8 = 7;

// Types  ================================================================================= Types
/// Represents a single schematic (lock or key) as a vector of column heights
type Schematic = Vec<u8>;

/// Contains all locks and keys parsed from input
struct Schematics {
    /// Vector of lock schematics (pin heights from top)
    locks: Vec<Schematic>,
    /// Vector of key schematics (pin heights from bottom)
    keys: Vec<Schematic>,
}

/// Represents either a lock or key schematic
enum SchematicClass {
    /// Lock schematic with pin heights from top
    Lock(Schematic),
    /// Key schematic with pin heights from bottom
    Key(Schematic),
}

impl SchematicClass {
    /// Converts a string representation of a schematic into either a Lock or Key
    ///
    /// # Arguments
    /// * `input` - String slice containing the schematic representation
    ///
    /// # Returns
    /// * `SchematicClass` - Either Lock or Key based on first character
    ///
    /// # Panics
    /// Panics if input contains invalid characters or is empty
    fn from_str(input: &str) -> Self {
        // Convert the 2D grid into column heights by counting '#' characters
        let schematic = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => 1u8,
                        '.' => 0u8,
                        _ => panic!("Invalid character in schematic"),
                    })
                    .collect::<Vec<_>>()
            })
            // Reduce by summing each column to get heights
            .reduce(|acc, row| acc.iter().zip(row).map(|(a, b)| a + b).collect())
            .expect("Empty schematic");

        // Classify as Lock or Key based on first character
        // Locks have '#' at top, Keys have '.' at top
        if input.starts_with('#') {
            Self::Lock(schematic)
        } else {
            Self::Key(schematic)
        }
    }
}

impl Schematics {
    /// Parses input string into Schematics struct
    ///
    /// # Arguments
    /// * `input` - String slice containing all schematics
    ///
    /// # Returns
    /// * `Schematics` - Struct containing separated locks and keys
    fn from_str(input: &str) -> Self {
        // Split input into individual schematics and partition into locks and keys
        let (locks, keys): (Vec<_>, Vec<_>) = input
            .split("\n\n")
            .map(SchematicClass::from_str)
            .partition_map(|class| match class {
                SchematicClass::Lock(schematic) => Either::Left(schematic),
                SchematicClass::Key(schematic) => Either::Right(schematic),
            });
        Self { locks, keys }
    }

    /// Counts number of valid lock/key pairs
    ///
    /// A valid pair is one where the sum of lock pin height and key height
    /// at each position is less than or equal to the grid height (7)
    ///
    /// # Returns
    /// * `usize` - Count of valid lock/key pairs
    fn count_match(&self) -> usize {
        self.locks
            .iter()
            .flat_map(|lock| {
                self.keys.iter().filter(|&key| {
                    // Check if all columns have valid combined height
                    lock.iter().zip(key).all(|(a, b)| a + b <= GRID_HEIGHT)
                })
            })
            .count()
    }
}

// Functions  =========================================================================== Functions
/// Solves part 1 of the puzzle
pub fn response_part_1() {
    println!("Day 25 - Part 1");
    let start = std::time::Instant::now();

    let schematics = Schematics::from_str(INPUT);
    let count = schematics.count_match();

    let duration = start.elapsed();

    println!("Count: {count}");
    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
}

// Tests  ================================================================================= Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_problem() {
        let input = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

        let schematics = Schematics::from_str(input);
        assert_eq!(schematics.count_match(), 3);
    }

    #[test]
    fn test_single_lock_key_matching() {
        let input = "\
###
.#.
.#.
...

...
#.#
###";

        let schematics = Schematics::from_str(input);
        assert_eq!(schematics.count_match(), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid character")]
    fn test_invalid_character() {
        let input = "\
#X#
.#.
...";
        Schematics::from_str(input);
    }
}
