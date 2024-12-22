///
/// # day_20.rs
/// Code for the day 20 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, str::FromStr};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_21.txt");
const NUMERIC_PAD: &str = "789\n456\n123\nX0A";
const DIRECTIONAL_PAD: &str = "X^A\n<v>";

type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Button {
    Up,
    Right,
    Down,
    Left,
    Press,
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl From<char> for Button {
    fn from(value: char) -> Self {
        match value {
            '^' => Button::Up,
            '>' => Button::Right,
            'v' => Button::Down,
            '<' => Button::Left,
            'A' => Button::Press,
            _ => panic!("Invalid button: {value}"),
        }
    }
}

impl From<Button> for char {
    fn from(value: Button) -> Self {
        match value {
            Button::Up => '^',
            Button::Right => '>',
            Button::Down => 'v',
            Button::Left => '<',
            Button::Press => 'A',
        }
    }
}

impl Button {
    ///
    /// # `from_movement`
    /// Get the button corresponding to a movement from one position to another.
    ///
    /// ## Arguments
    /// * `from` - The starting position
    /// * `to` - The ending position
    ///
    /// ## Returns
    /// * `Button` - The button corresponding to the movement
    fn from_movement(from: Position, to: Position) -> Self {
        use std::cmp::Ordering;
        match (to.0.cmp(&from.0), to.1.cmp(&from.1)) {
            (Ordering::Less, _) => Button::Left,
            (Ordering::Greater, _) => Button::Right,
            (_, Ordering::Less) => Button::Up,
            (_, Ordering::Greater) => Button::Down,
            (Ordering::Equal, Ordering::Equal) => Button::Press,
        }
    }
}

#[derive(Debug)]
struct KeypadGrid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl FromStr for KeypadGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = grid[0].len();

        Ok(KeypadGrid {
            grid,
            width,
            height,
        })
    }
}

impl KeypadGrid {
    ///
    /// # `neighbors`
    /// Get the walkable neighbors of a given position.
    ///
    /// ## Arguments
    /// * `pos` - The position to get the neighbors from
    ///
    /// ## Returns
    /// * `Vec<Position>` - The walkable neighbors of the given position
    fn neighbors(&self, pos: Position) -> Vec<Position> {
        let (x, y) = pos;
        let mut neighbors = Vec::new();

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x + 1 < self.width {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y + 1 < self.height {
            neighbors.push((x, y + 1));
        }

        neighbors
            .into_iter()
            .filter(|&(nx, ny)| self.grid[ny][nx] != 'X')
            .collect()
    }

    ///
    /// # `get`
    /// Get the character at a given position.
    ///
    /// ## Arguments
    /// * `pos` - The position to get the character from
    ///
    /// ## Returns
    /// * `char` - The character at the given position
    fn get(&self, pos: Position) -> char {
        self.grid[pos.1][pos.0]
    }

    /// # `find_char`
    /// Find the position of a given character in the grid.
    ///
    /// ## Arguments
    /// * `c` - The character to find
    ///
    /// ## Returns
    /// * `Option<Position>` - The position of the character if found
    fn find_char(&self, c: char) -> Option<Position> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == c {
                    return Some((x, y));
                }
            }
        }

        None
    }
}

struct PuzzleSolver {
    numeric_pad: KeypadGrid,
    directional_pad: KeypadGrid,
}

impl PuzzleSolver {
    pub fn new() -> Self {
        Self {
            numeric_pad: KeypadGrid::from_str(NUMERIC_PAD).unwrap(),
            directional_pad: KeypadGrid::from_str(DIRECTIONAL_PAD).unwrap(),
        }
    }

    ///
    /// # `build_press_costs`
    /// Build a matrix of costs to press a button from another button.
    ///
    /// ## Algorithm
    /// For each button, perform a BFS to find the shortest path to all other buttons
    /// The cost of pressing a button is the number of times it takes to press the buttons
    /// from the current button to the target button.
    ///
    /// ## Arguments
    /// * `level` - The number of times to press the buttons
    ///
    /// ## Returns
    /// * `[[usize; 5]; 5]` - The matrix of costs to press a button from another button
    fn build_press_costs(&self, level: i32) -> [[usize; 5]; 5] {
        if level == 0 {
            return [[1; 5]; 5];
        }

        let mut current_costs = [[0; 5]; 5];
        let previous_costs = self.build_press_costs(level - 1);

        for from in [
            Button::Up,
            Button::Right,
            Button::Down,
            Button::Left,
            Button::Press,
        ] {
            let mut seen = [false; 5];
            let from_pos = self.directional_pad.find_char(from.into()).unwrap();
            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0), from_pos, Button::Press));

            while let Some((Reverse(cost), pos, button)) = queue.pop() {
                let current_button = Button::try_from(self.directional_pad.get(pos)).unwrap();

                if cost > 0 && button == Button::Press {
                    if current_costs[from as usize][current_button as usize] == 0 {
                        current_costs[from as usize][current_button as usize] = cost;
                    }
                } else {
                    queue.push((
                        Reverse(cost + previous_costs[button as usize][Button::Press as usize]),
                        pos,
                        Button::Press,
                    ));
                }

                seen[current_button as usize] = true;

                for next_pos in self.directional_pad.neighbors(pos) {
                    let next_button = Button::try_from(self.directional_pad.get(next_pos)).unwrap();
                    if seen[next_button as usize] {
                        continue;
                    }
                    let movement = Button::from_movement(pos, next_pos);
                    queue.push((
                        Reverse(cost + previous_costs[button as usize][movement as usize]),
                        next_pos,
                        movement,
                    ));
                }
            }
        }
        current_costs
    }

    ///
    /// # `shortest_path`
    /// Find the shortest path between two buttons.
    ///
    /// ## Algorithm
    /// Perform a BFS
    ///
    /// ## Arguments
    /// * `press_costs` - The matrix of costs to press a button from another button
    /// * `from` - The starting button
    /// * `to` - The ending button
    ///
    /// ## Returns
    /// * `usize` - The cost of the shortest path between the two buttons
    fn shortest_path(&self, press_costs: [[usize; 5]; 5], from: char, to: char) -> usize {
        let start = self.numeric_pad.find_char(from).unwrap();
        let end = self.numeric_pad.find_char(to).unwrap();

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), start, Button::Press));

        while let Some((Reverse(cost), pos, button)) = queue.pop() {
            if pos == end {
                if button == Button::Press {
                    return cost;
                }
                queue.push((
                    Reverse(cost + press_costs[button as usize][Button::Press as usize]),
                    pos,
                    Button::Press,
                ));
            }

            for next_pos in self.numeric_pad.neighbors(pos) {
                let next_button = Button::from_movement(pos, next_pos);
                queue.push((
                    Reverse(cost + press_costs[button as usize][next_button as usize]),
                    next_pos,
                    next_button,
                ));
            }
        }
        panic!("No path found between {from} and {to}");
    }

    ///
    /// # `calculate_code_complexity`
    /// Calculate the complexity of a code.
    ///
    /// ## Arguments
    /// * `press_costs` - The matrix of costs to press a button from another button
    /// * `code` - The code to calculate the complexity of
    ///
    /// ## Returns
    /// * `usize` - The complexity of the code
    fn calculate_code_complexity(&self, press_costs: [[usize; 5]; 5], code: &str) -> usize {
        let moves = format!("A{code}");
        let total_cost: usize = moves
            .as_bytes()
            .windows(2)
            .map(|w| self.shortest_path(press_costs, w[0] as char, w[1] as char))
            .sum();

        if let Some(n) = code.strip_suffix('A') {
            let numeric_value: usize = n.parse().unwrap();
            total_cost * numeric_value
        } else {
            panic!("Invalid input: {code}");
        }
    }
}

pub fn response_part_1() {
    println!("Day 21 - Part 1");

    let start = std::time::Instant::now();
    let solver = PuzzleSolver::new();

    let press_costs = solver.build_press_costs(2);

    let result: usize = INPUT
        .trim()
        .lines()
        .map(|line| solver.calculate_code_complexity(press_costs, line))
        .sum();

    let duration = start.elapsed();

    println!("Result: {}", result);
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 21 - Part 2");
    let start = std::time::Instant::now();

    let solver = PuzzleSolver::new();
    let press_costs = solver.build_press_costs(25);

    let result: usize = INPUT
        .trim()
        .lines()
        .map(|line| solver.calculate_code_complexity(press_costs, line))
        .sum();

    let duration = start.elapsed();

    println!("Result: {}", result);
    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    response_part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_conversion() {
        assert_eq!(Button::from('^'), Button::Up);
        assert_eq!(Button::from('A'), Button::Press);
    }

    #[test]
    fn test_keypad_grid() {
        let grid = KeypadGrid::from_str(NUMERIC_PAD).unwrap();
        assert_eq!(grid.find_char('5'), Some((1, 1)));
        assert_eq!(grid.find_char('X'), Some((0, 3)));
    }
}
