
# Advent of Code 2024
```
        .
\_____)\_____
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
|-----|--------|--------|
| [Day 1](src/bin/day_01.rs) | 68.375µs | 78.333µs |
| [Day 2](src/bin/day_02.rs) | 176.541µs | 371.666µs |
| [Day 3](src/bin/day_03.rs) | 2.400917ms | 3.335458ms |
| [Day 4](src/bin/day_04.rs) | 6.48275ms | 1.396709ms |
| [Day 5](src/bin/day_05.rs) | 4.281167ms | 15.094ms |
| [Day 6](src/bin/day_06.rs) | 2.106666ms | 30.646914917s |
| [Day 7](src/bin/day_07.rs) | 72.25425ms | 638.079083ms |
| [Day 8](src/bin/day_08.rs) | 42.917µs | 80.25µs |
| [Day 9](src/bin/day_09.rs) | 614.292µs | 482.333µs |

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
This project is open source and available under the MIT License.
