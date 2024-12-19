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
| [Day 1](src/bin/day_01.rs) | 62.79µs | 79.17µs |
| [Day 2](src/bin/day_02.rs) | 176.54µs | 371.67µs |
| [Day 3](src/bin/day_03.rs) | 2.40ms | 3.34ms |
| [Day 4](src/bin/day_04.rs) | 6.48ms | 1.40ms |
| [Day 5](src/bin/day_05.rs) | 4.01ms | 17.74ms |
| [Day 6](src/bin/day_06.rs) | 2.11ms | 30.65s |
| [Day 7](src/bin/day_07.rs) | 72.25ms | 638.08ms |
| [Day 8](src/bin/day_08.rs) | 42.92µs | 80.25µs |
| [Day 9](src/bin/day_09.rs) | 614.29µs | 482.33µs |
| [Day 10](src/bin/day_10.rs) | 873.50µs | 846.79µs |
| [Day 11](src/bin/day_11.rs) | 364.83µs | 21.11ms |
| [Day 12](src/bin/day_12.rs) | 2.12ms | 9.39ms |
| [Day 13](src/bin/day_13.rs) | 85.04ms | 66.92ms |
| [Day 14](src/bin/day_14.rs) | 71.84ms | 55.64ms |
| [Day 15](src/bin/day_15.rs) | 372.83µs | 453.42µs |
| [Day 16](src/bin/day_16.rs) | 743.59ms | 672.50ms |
| [Day 17](src/bin/day_17.rs) | 577.62µs | 140.04µs |
| [Day 18](src/bin/day_18.rs) | 810.21µs | 1.37s |
| [Day 19](src/bin/day_19.rs) | 2.62ms | 39.50ms |


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