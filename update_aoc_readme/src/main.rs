// Imports  ==============================================================================  Imports

use std::{path::Path, process::Command, str::FromStr};

use regex::Regex;

// Variables  =========================================================================== Variables
#[derive(Debug, Clone)]
struct Time {
    number: f32,
    unit: String,
}

#[derive(Debug, Clone)]
struct Day {
    number: u8,
    part_1: Option<Time>,
    part_2: Option<Time>,
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time_regex = Regex::new(r"(?P<value>\d+\.\d+)(?P<unit>\w+)").unwrap();
        let captures = time_regex.captures(s).unwrap();

        Ok(Time {
            number: captures
                .name("value")
                .unwrap()
                .as_str()
                .parse::<f32>()
                .unwrap(),
            unit: captures.name("unit").unwrap().as_str().to_string(),
        })
    }
}

impl Time {
    fn to_string(&self) -> String {
        format!("{:.2}{}", self.number, self.unit)
    }
}

// Functions  =========================================================================== Functions
///
/// # `git_staged_files_to_days`
/// Get the staged files from git and return a vector of `Day` struct.
///
/// ## Returns
/// * `Vec<Day>`: Vector of `Day` struct.
fn git_staged_files_to_days() -> Vec<Day> {
    let git_command = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--name-only")
        .output()
        .expect("Failed to execute git diff --cached --name-only");

    let git_output =
        String::from_utf8(git_command.stdout).expect("Failed to convert git output to string");

    let day_regex = Regex::new(r"src/bin/day_(\d+)\.rs").unwrap();

    git_output
        .lines()
        .filter_map(|line| day_regex.captures(line))
        .map(|captures| {
            let day_number = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();

            Day {
                number: day_number,
                part_1: None,
                part_2: None,
            }
        })
        .collect()
}

///
/// # `get_existing_days_in_readme`
/// Get the existing days in the README.md file.
///
/// ## Returns
/// * `Vec<Day>`: Vector of `Day` struct.
fn get_existing_days_in_readme() -> Vec<Day> {
    let readme_path = Path::new("README.md");

    if !readme_path.exists() {
        return vec![];
    }

    let readme_content = std::fs::read_to_string(readme_path).unwrap();

    let day_regex =
        Regex::new(r"\| \[Day (?P<day_number>\d+)\]\(src/bin/day_(?:\d+)\.rs\) \| (?P<part_1>.*?) \| (?P<part_2>.*?) \|")
            .unwrap();

    day_regex
        .captures_iter(&readme_content)
        .map(|captures| {
            let day_number = captures
                .name("day_number")
                .unwrap()
                .as_str()
                .parse::<u8>()
                .unwrap();

            let part_1 = captures
                .name("part_1")
                .map(|time| Time::from_str(time.as_str()).unwrap());

            let part_2 = captures
                .name("part_2")
                .map(|time| Time::from_str(time.as_str()).unwrap());

            Day {
                number: day_number,
                part_1,
                part_2,
            }
        })
        .collect()
}

///
/// # `time_execution`
/// Time the execution of a day.
///
/// ## Arguments
/// * `day`: `Day` struct.
///
/// ## Returns
/// * `Day`: `Day` struct with the execution time.
fn time_execution(day: Day) -> Day {
    let day_path = format!("day_{:02}", day.number);

    let bin_run_command = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--bin")
        .arg(&day_path)
        .output()
        .expect("Failed to execute cargo run");

    let time_output = String::from_utf8(bin_run_command.stdout).unwrap();

    let time_regex = Regex::new(r"Duration: (?P<value>\d+\.\d+)(?P<unit>\w+)").unwrap();
    let times = time_regex
        .captures_iter(&time_output)
        .map(|captures| Time {
            number: captures
                .name("value")
                .unwrap()
                .as_str()
                .parse::<f32>()
                .unwrap(),
            unit: captures.name("unit").unwrap().as_str().to_string(),
        })
        .collect::<Vec<Time>>();

    Day {
        number: day.number,
        part_1: times.first().cloned(),
        part_2: times.get(1).cloned(),
    }
}

///
/// # `update_readme`
/// Update the README.md file with the new days.
fn update_readme() {
    let readme_path = Path::new("README.md");

    let existing_days = get_existing_days_in_readme();
    let staged_days = git_staged_files_to_days();

    let days_to_update = staged_days
        .iter()
        .filter(|staged_day| {
            // Get corresponding existing day if it exists
            if let Some(existing_day) = existing_days.iter().find(|d| d.number == staged_day.number)
            {
                // Update if either part is missing
                existing_day.part_1.is_none() || existing_day.part_2.is_none()
            } else {
                // Include new days
                true
            }
        })
        .map(|day| time_execution(day.clone()))
        .collect::<Vec<Day>>();

    let mut final_days: Vec<Day> = existing_days
        .iter()
        .map(|existing_day| {
            // If this day needs updating, use the updated version
            if let Some(updated_day) = days_to_update
                .iter()
                .find(|d| d.number == existing_day.number)
            {
                updated_day.clone()
            } else {
                existing_day.clone()
            }
        })
        .chain(
            // Add any new days that weren't in existing_days
            days_to_update
                .iter()
                .filter(|day| !existing_days.iter().any(|d| d.number == day.number))
                .cloned(),
        )
        .collect();

    final_days.sort_by(|a, b| a.number.cmp(&b.number));

    if days_to_update.is_empty() {
        return;
    }

    let mut new_content = String::from("# Advent of Code 2024
```
        .
\\_____)\\_____
/--v____ __`< My Rust solutions to the Advent of Code 2024 challenges
    )/
    '
```

## Overview
This repository contains my solutions to the [Advent of Code 2024](https://adventofcode.com/2024) challenges, implemented in Rust.

## Project Structure
- `src/bin/`: Contains the daily challenge solutions
- `src/lib.rs`: Common utilities and helper functions
- `src/points.rs`: Point-related utilities for geometric calculations
- `data/inputs/`: Input files for each day's challenge (not included in repository)

## Solutions
| Day | Part 1 | Part 2 |
|-----|--------|--------|\n");

    let final_days_content = final_days
        .iter()
        .map(|day| {
            format!(
                "| [Day {}](src/bin/day_{:02}.rs) | {} | {} |\n",
                day.number,
                day.number,
                day.part_1
                    .as_ref()
                    .map_or("".to_string(), |time| time.to_string()),
                day.part_2
                    .as_ref()
                    .map_or("".to_string(), |time| time.to_string())
            )
        })
        .collect::<String>();

    new_content.push_str(&final_days_content);

    new_content.push_str(
        "\n
## Running the Solutions

To run a specific day's solution:
```bash
cargo run --release --bin day_01
```

To run a specific day's solution tests:
```bash
cargo test --bin day_01
```

To run all tests:
```bash
cargo test
```


## License
This project is open source and available under the MIT License.",
    );

    std::fs::write(readme_path, new_content).unwrap();
}

fn main() {
    let current = Path::new("/Users/tom_planche/Desktop/Prog/Rust/all_aoc/aoc_2024");
    std::env::set_current_dir(current).unwrap();

    update_readme();
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_from_str() {
        let time_str = "0.00s";
        let time: Time = time_str.parse().unwrap();

        assert_eq!(time.number, 0.00);
        assert_eq!(time.unit, "s");
    }

    #[test]
    fn test_time_from_str_v2() {
        let time_str = "176.541µs";
        let time: Time = time_str.parse().unwrap();

        assert_eq!(time.number, 176.541);
        assert_eq!(time.unit, "µs");
    }
}
