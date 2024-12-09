# Update AOC README

A Rust utility to automatically update the README.md file of Advent of Code solutions repository by timing the execution of new solutions and formatting the results in a table.

## Features

- Detects newly staged solution files using git
- Times the execution of new solutions
- Maintains a formatted table of solutions with their execution times
- Preserves existing entries while adding new ones
- Supports different time units (seconds, microseconds, etc.)

## How it Works

1. Checks git for newly staged solution files
2. Retrieves existing solutions from README.md
3. Times the execution of new solutions
4. Merges existing and new solutions
5. Formats and updates the README.md with the combined results

## Usage

I made a pre-commit hook that runs the utility before each commit.
It looks like this:
```bash
#!/usr/bin/env bash
# run the `./update_aoc_readme/target/release/update_aoc_readme` binary to update the README.md
# when files in `./problems/id_*` are changed.

# get the list of files that have been changed
# since the last commit
files=$(git diff --cached --name-only)

# check if any of the files are in the `./problems/id_*` directory
if [[ $files == *"src/bin/day_"* ]]; then
    # if so, run the `update_readme` binary
    ./update_aoc_readme/target/release/update_aoc_readme

    # add the changes to the commit
    git add README.md
fi

# continue with the commit
exit 0
```
