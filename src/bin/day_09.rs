use std::cmp::Reverse;
///
/// # day_09.rs
/// Code for the day 09 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_09.txt");

/// Represents a file on the virtual disk
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
struct FileDescriptor {
    /// Unique identifier for the file
    file_id: i64,
    /// Starting position of file on disk
    pointer: usize,
    /// Size of file in blocks
    size: usize,
}

#[derive(Clone, Copy)]
enum Block {
    /// Occupied block containing file ID
    Filled(usize),
    /// Empty/free block
    Empty,
}

impl Block {
    fn is_empty(&self) -> bool {
        if let Block::Empty = &self {
            return true;
        }
        false
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Filled(val) => write!(f, "{}", val),
            Self::Empty => write!(f, "."),
        }
    }
}

struct Disk {
    blocks: Vec<Block>,
    files: Vec<FileDescriptor>,
    free_spaces: [BinaryHeap<Reverse<usize>>; 10],
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::new();
        let mut files = Vec::new();
        let mut file_index = 0;
        let mut free_spaces: [BinaryHeap<Reverse<usize>>; 10] = [
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
        ];

        for line in s.lines() {
            for (i, c) in line.chars().enumerate() {
                let val = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    for _ in 0..val {
                        blocks.push(Block::Filled(file_index));
                    }
                    files.push(FileDescriptor {
                        file_id: file_index as i64,
                        pointer: blocks.len() - val,
                        size: val,
                    });
                    file_index += 1;
                } else {
                    for _ in 0..val {
                        blocks.push(Block::Empty);
                    }
                    free_spaces[val].push(Reverse(blocks.len() - val));
                }
            }
        }

        Ok(Disk {
            blocks,
            files,
            free_spaces,
        })
    }
}

impl Disk {
    ///
    /// # get_first_empty
    /// Finds the first empty block starting from given position
    ///
    /// ## Arguments
    /// * `start` - Position to start searching from
    ///
    /// ## Returns
    /// * `usize` - Index of first empty block
    fn get_first_empty(&self, start: usize) -> usize {
        self.blocks[start..]
            .iter()
            .position(|block| block.is_empty())
            .unwrap()
            + start
    }

    ///
    /// # rearrange
    /// Reorganizes blocks to optimize disk space
    /// Moves all filled blocks to the right and empty blocks to the left
    fn rearrange(&mut self) {
        let mut left = self.get_first_empty(0);
        let mut right = self.blocks.len() - 1;

        while left < right {
            self.blocks.swap(right, left);
            left = self.get_first_empty(left);
            right -= 1;
        }
    }

    ///
    /// # calc_checksum
    /// Calculates checksum of disk arrangement
    /// Checksum is sum of (block_index * file_id) for all filled blocks
    ///
    /// ## Returns
    /// * `usize` - Calculated checksum value
    fn rearrange_files(&mut self) {
        // Iterate through files in reverse order to process larger files first
        for file in self.files.iter().rev() {
            // Initialize variables to track best free space location
            let mut free_space = file.pointer; // Current file position as default
            let mut old_size = 0; // Size of free space block we'll use

            // Look for free spaces larger than current file size
            for i in file.size..10 {
                // Check if there's a free space of size i available
                if let Some(&Reverse(free)) = self.free_spaces[i].peek() {
                    // If this free space is earlier in the disk than current best
                    if free < free_space {
                        free_space = free; // Update best free space location
                        old_size = i; // Store size of this free space block
                    }
                }
            }

            // If we found a better position (old_size > 0), move the file
            if old_size != 0 {
                // Swap blocks to move file to new location
                for i in 0..file.size {
                    self.blocks.swap(free_space + i, file.pointer + i);
                }

                // Remove used free space from tracking
                self.free_spaces[old_size].pop();

                // Add new free space where file used to be
                let new_free_space = Reverse(free_space + file.size);
                self.free_spaces[old_size - file.size].push(new_free_space);
            }
        }
    }

    fn calc_checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, v)| match v {
                Block::Filled(val) => i * val,
                Block::Empty => 0,
            })
            .filter(|i| *i != 0)
            .sum()
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            write!(f, "{}", block).unwrap();
        }

        write!(f, "")
    }
}

// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 09 - Part 1");
    let start = std::time::Instant::now();

    let mut disk = INPUT.parse::<Disk>().unwrap();
    disk.rearrange();
    let checksum = disk.calc_checksum();

    let duration = start.elapsed();

    println!("Checksum: {checksum}");
    println!("Duration: {duration:?}\n");
}

pub fn response_part_2() {
    println!("Day 09 - Part 2");
    let start = std::time::Instant::now();

    let mut disk = INPUT.parse::<Disk>().unwrap();
    disk.rearrange_files();

    let checksum = disk.calc_checksum();
    let duration = start.elapsed();

    println!("Checksum: {checksum}");
    println!("Duration: {:?}", duration);
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    const DUMMT_INPUT: &str = "12345";
    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_disk_from_dummy_str() {
        let disk = DUMMT_INPUT.parse::<Disk>().unwrap();

        assert_eq!(disk.blocks.len(), 15);
        assert_eq!(disk.files.len(), 3);
    }

    #[test]
    fn test_disk_from_example_str() {
        let disk = EXAMPLE_INPUT.parse::<Disk>().unwrap();

        assert_eq!(disk.blocks.len(), 42);
        assert_eq!(disk.files.len(), 10);
    }

    #[test]
    fn test_checksum_dummy() {
        let mut disk = DUMMT_INPUT.parse::<Disk>().unwrap();
        disk.rearrange();

        assert_eq!(disk.calc_checksum(), 60);
    }

    #[test]
    fn test_checksum_example() {
        let mut disk = EXAMPLE_INPUT.parse::<Disk>().unwrap();
        disk.rearrange();

        assert_eq!(disk.calc_checksum(), 1928);
    }
}
