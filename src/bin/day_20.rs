///
/// # day_20.rs
/// Code for the day 20 of the Advent of Code challenge year 2024
///
// Imports  ==============================================================================  Imports
use aoc_2024::Point;
use rayon::prelude::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_20.txt");

type MyPoint = Point<usize>;

#[derive(Debug, Clone)]
struct Maze {
    grid: Vec<Vec<char>>,
    start: MyPoint,
    end: MyPoint,
}

impl Maze {
    ///
    /// # `get`
    /// Get the character at a given point in the maze.
    ///
    /// ## Arguments
    /// * `p` - The point to get the character from
    ///
    /// ## Returns
    /// * `char` - The character at the given point
    fn get(&self, p: MyPoint) -> char {
        let x = p.x as usize;
        let y = p.y as usize;

        self.grid
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or('#')
    }

    ///
    /// # `is_walkable`
    /// Check if a given point is walkable in the maze.
    ///
    /// ## Arguments
    /// * `point` - The point to check
    ///
    /// ## Returns
    /// * `bool` - Whether the point is walkable
    fn is_walkable(&self, point: MyPoint) -> bool {
        self.get(point) != '#'
    }

    ///
    /// # `neighbors`
    /// Get the walkable neighbors of a given point.
    ///
    /// ## Arguments
    /// * `p` - The point to get the neighbors from
    ///
    /// ## Returns
    /// * `Vec<MyPoint>` - The walkable neighbors of the given point
    fn neighbors(&self, p: MyPoint) -> Vec<MyPoint> {
        [
            MyPoint::new(p.x + 1, p.y),
            MyPoint::new(p.x - 1, p.y),
            MyPoint::new(p.x, p.y + 1),
            MyPoint::new(p.x, p.y - 1),
        ]
        .into_iter()
        .filter(|&p| self.is_walkable(p))
        .collect()
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut start = MyPoint::new(0, 0);
        let mut end = MyPoint::new(0, 0);

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' | '.' => row.push(c),
                    'S' => {
                        row.push('.');
                        start = MyPoint::new(x, y);
                    }
                    'E' => {
                        row.push('.');
                        end = MyPoint::new(x, y);
                    }
                    _ => return Err(()),
                }
            }
            grid.push(row);
        }

        Ok(Maze { grid, start, end })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathState {
    cost: usize,
    position: MyPoint,
}

impl PathState {
    fn new(cost: usize, position: MyPoint) -> Self {
        Self { cost, position }
    }
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct AStar<'a> {
    maze: &'a Maze,
    frontier: BinaryHeap<PathState>,
    came_from: HashMap<MyPoint, Option<MyPoint>>,
    cost_so_far: HashMap<MyPoint, usize>,
}

impl<'a> AStar<'a> {
    fn new(maze: &'a Maze) -> Self {
        let mut frontier = BinaryHeap::new();
        frontier.push(PathState::new(0, maze.start));

        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();

        came_from.insert(maze.start, None);
        cost_so_far.insert(maze.start, 0);

        Self {
            maze,
            frontier,
            came_from,
            cost_so_far,
        }
    }

    ///
    /// # `find_path`
    /// Find the shortest path from the start to the end of the maze.
    ///
    /// ## Algorithm
    /// A* algorithm to find the shortest path from the start to the end of the maze.
    ///
    /// ## Returns
    /// * `Option<(usize, Vec<MyPoint>)>` - The cost of the path and the path itself
    fn find_path(&mut self) -> Option<(usize, Vec<MyPoint>)> {
        while let Some(current) = self.frontier.pop() {
            if current.position == self.maze.end {
                break;
            }

            for next in self.maze.neighbors(current.position) {
                let new_cost = self.cost_so_far[&current.position] + 1;

                if !self.cost_so_far.contains_key(&next) || new_cost < self.cost_so_far[&next] {
                    self.cost_so_far.insert(next, new_cost);

                    let priority = new_cost + next.manhattan_distance(&self.maze.end);

                    self.frontier.push(PathState::new(priority, next));
                    self.came_from.insert(next, Some(current.position));
                }
            }
        }

        self.reconstruct_path()
    }

    ///
    /// # `reconstruct_path`
    /// Reconstruct the path from the start to the end of the maze.
    ///
    /// ## Returns
    /// * `Option<(usize, Vec<MyPoint>)>` - The cost of the path and the path itself
    fn reconstruct_path(&self) -> Option<(usize, Vec<MyPoint>)> {
        let mut path = vec![self.maze.end];
        let mut current = self.maze.end;

        while current != self.maze.start {
            if let Some(Some(prev)) = self.came_from.get(&current) {
                current = *prev;

                path.push(current);
            } else {
                return None;
            }
        }
        path.reverse();

        Some((self.cost_so_far[&self.maze.end], path))
    }
}

struct PathFinder {
    path: Vec<MyPoint>,
}

impl PathFinder {
    fn new(path: Vec<MyPoint>) -> Self {
        Self { path }
    }

    ///
    /// # `find_cheats`
    /// Find the number of possible cheats in the path.
    ///
    /// ## Arguments
    /// * `max_cheat_time` - The maximum time to cheat
    fn find_cheats(&self, max_cheat_time: usize, min_savings: usize) -> usize {
        (0..self.path.len())
            .par_bridge() // Parallelize the loop
            .map(|start_time| self.find_cheats_from(max_cheat_time, min_savings, start_time))
            .sum()
    }

    ///
    /// # `find_cheats_from`
    /// Find the number of possible cheats in the path from a given start time.
    ///
    /// ## Arguments
    /// * `max_cheat_time` - The maximum time to cheat
    /// * `min_savings` - The minimum time to save
    /// * `start_time` - The start time to find cheats from
    ///
    /// ## Returns
    /// * `usize` - The number of possible cheats
    fn find_cheats_from(
        &self,
        max_cheat_time: usize,
        min_savings: usize,
        start_time: usize,
    ) -> usize {
        let mut viable = 0;
        let cheat_start = self.path[start_time];

        if start_time > self.path.len() - min_savings {
            return 0;
        }

        let mut normal_end_time = start_time + min_savings;
        while normal_end_time < self.path.len() {
            let cheat_end = self.path[normal_end_time];
            let cheat_dist = cheat_start.manhattan_distance(&cheat_end);

            if cheat_dist > max_cheat_time {
                normal_end_time += cheat_dist - max_cheat_time;
            } else {
                let cheat_end_time = start_time + cheat_dist;
                let savings = normal_end_time - cheat_end_time;

                if savings >= min_savings {
                    viable += 1;
                }

                normal_end_time += 1;
            }
        }
        viable
    }
}

pub fn response_part_1() {
    println!("Day 19 - Part 1");
    let start = std::time::Instant::now();

    let maze = Maze::from_str(INPUT).unwrap();
    let (_, normal_path) = AStar::new(&maze).find_path().unwrap();
    let path_finder = PathFinder::new(normal_path);
    let cheats = path_finder.find_cheats(2, 100);

    let duration = start.elapsed();

    println!("cheats: {cheats}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 19 - Part 2");
    let start = std::time::Instant::now();

    let maze = Maze::from_str(INPUT).unwrap();
    let (_, normal_path) = AStar::new(&maze).find_path().unwrap();
    let path_finder = PathFinder::new(normal_path);
    let cheats = path_finder.find_cheats(20, 100);

    let duration = start.elapsed();

    println!("cheats: {cheats}");
    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    response_part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_from_str() {
        let maze = Maze::from_str(EXAMPLE_INPUT).unwrap();
        assert_eq!(maze.grid.len(), 15);
        assert_eq!(maze.grid[0].len(), 15);
        assert_eq!(maze.start, MyPoint::new(1, 3));
        assert_eq!(maze.end, MyPoint::new(5, 7));
    }
}
