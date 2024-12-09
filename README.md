
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
| [Day 1](src/bin/day_01.rs) | 68.875µs | 86.209µs |
| [Day 2](src/bin/day_02.rs) | 177.875µs | 333.292µs |
| [Day 3](src/bin/day_03.rs) | 3.197125ms | 3.065792ms |
| [Day 4](src/bin/day_04.rs) | 6.011917ms | 1.723875ms |
| [Day 5](src/bin/day_05.rs) | 4.086584ms | 17.141209ms |
| [Day 6](src/bin/day_06.rs) | 2.699833ms | 31.103930042s |
| [Day 7](src/bin/day_07.rs) | 80.280875ms | 644.997291ms |
| [Day 8](src/bin/day_08.rs) | 44.208µs | 81.333µs |
| [Day 9](src/bin/day_09.rs) | 588.291µs | 479.583µs |

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
