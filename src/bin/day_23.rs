///
/// # Day 23: LAN Party
/// Solution for the Advent of Code 2024 Day 23 challenge.
/// Finds sets of three interconnected computers in a network where at least
/// one computer name starts with 't'.
///
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

// Constants ============================================================================ Constants
const INPUT: &str = include_str!("../../data/inputs/day_23.txt");

// Types ================================================================================== Types
/// Represents an undirected graph using adjacency lists
#[derive(Debug)]
struct Graph {
    adj_list: HashMap<String, HashSet<String>>,
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::new();

        for line in s.trim().lines() {
            let (from, to) = line.split_once('-').unwrap();
            graph.add_edge(from, to);
        }

        Ok(graph)
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    ///
    /// # `add_edge`
    /// Adds an undirected edge between two nodes
    ///
    /// ## Arguments
    /// * `from` - First node name
    /// * `to` - Second node name
    fn add_edge(&mut self, from: &str, to: &str) {
        self.adj_list
            .entry(from.to_string())
            .or_insert_with(HashSet::new)
            .insert(to.to_string());

        self.adj_list
            .entry(to.to_string())
            .or_insert_with(HashSet::new)
            .insert(from.to_string());
    }

    ///
    /// # `find_triads_with_t`
    /// Finds all sets of three interconnected nodes (triads)
    /// where at least one node starts with 't'
    ///
    /// ## Returns
    /// * `HashSet<Vec<String>>` - Set of triads, each represented as a sorted vector of node names
    fn find_triads_with_t(&self) -> HashSet<Vec<String>> {
        let mut triads = HashSet::new();

        // For each node and its neighbors
        for (node, neighbors) in &self.adj_list {
            // For each pair of neighbors
            for n1 in neighbors {
                for n2 in neighbors {
                    if n1 >= n2 {
                        continue;
                    } // Avoid duplicates

                    // Check if n1 and n2 are connected
                    if self.adj_list[n1].contains(n2) {
                        // Create and sort the triad for consistent comparison
                        let mut triad = vec![node.clone(), n1.clone(), n2.clone()];
                        triad.sort();

                        // Add only if at least one node starts with 't'
                        if triad.iter().any(|n| n.starts_with('t')) {
                            triads.insert(triad);
                        }
                    }
                }
            }
        }

        triads
    }
}

// Functions ============================================================================ Functions
pub fn response_part_1() {
    println!("Day 23 - Part 1");
    let start = std::time::Instant::now();

    let graph: Graph = INPUT.parse().unwrap();
    let result = graph.find_triads_with_t().len();

    let duration = start.elapsed();

    println!("Result: {result}");
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 23 - Part 2");
    let start = std::time::Instant::now();

    let duration = start.elapsed();
    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    //response_part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_solve_part1() {
        let graph: Graph = TEST_INPUT.parse().unwrap();
        let result = graph.find_triads_with_t().len();

        assert_eq!(result, 7);
    }

    #[test]
    fn test_graph_construction() {
        let graph: Graph = "a-b\nb-c\nc-a".parse().unwrap();

        assert!(graph.adj_list["a"].contains("b"));
        assert!(graph.adj_list["b"].contains("c"));
        assert!(graph.adj_list["c"].contains("a"));
    }
}
