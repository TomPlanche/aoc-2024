///
/// # day_03.rs
/// Code for the day 03 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::str::FromStr;

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_03.txt");

struct Multiplications {
    muls: Vec<(usize, usize)>,
}

impl FromStr for Multiplications {
    type Err = ();

    ///
    /// # from_str Parse the input string to a Multiplications struct. This is
    /// used for the part 1 of the challenge. A `mul` function is defined as
    /// `mul(a, b)` where `a` and `b` are integers between 0 and 999, hence the `\d{1,3}`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut muls = Vec::new();

        let mul_regex = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

        for cap in mul_regex.captures_iter(s) {
            let mut nums = cap[0].split(|c| c == '(' || c == ',' || c == ')');

            nums.next(); // ignore the first element

            let a = nums.next().unwrap().parse().unwrap();
            let b = nums.next().unwrap().parse().unwrap();

            muls.push((a, b));
        }

        Ok(Multiplications { muls })
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 03 - Part 1");

    let sum = INPUT
        .parse::<Multiplications>()
        .unwrap()
        .muls
        .iter()
        .map(|(a, b)| a * b)
        .sum::<usize>();

    println!("The sum of all multiplications is: {}", sum);
}

pub fn response_part_2() {
    println!("Day 03 - Part 2");
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
