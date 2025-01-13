///
/// # Day 23: LAN Party
/// Solution for the Advent of Code 2024 Day 23 challenge.
/// Finds sets of three interconnected computers in a network where at least
/// one computer name starts with 't'
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

    ///
    /// # `find_maximum_clique`
    /// Finds the maximum clique in the graph using the Bron-Kerbosch algorithm
    /// with pivoting for better performance
    ///
    /// ## Returns
    /// * `Vec<String>` - Sorted vector of node names in the maximum clique
    fn find_maximum_clique(&self) -> Vec<String> {
        let mut max_clique = HashSet::new();
        let mut candidates: HashSet<_> = self.adj_list.keys().cloned().collect();
        let mut excluded = HashSet::new();

        self.bron_kerbosch(
            &mut HashSet::new(),
            &mut candidates,
            &mut excluded,
            &mut max_clique,
        );

        let mut result: Vec<_> = max_clique.into_iter().collect();
        result.sort();

        result
    }

    ///
    /// # `bron_kerbosch`
    /// Recursive helper for the Bron-Kerbosch algorithm with pivoting
    ///
    /// ## Algorithm
    /// The Bron-Kerbosch algorithm finds all maximal cliques in an undirected graph using
    /// recursive backtracking with three sets:
    /// 1. `clique`: vertices in the current clique being built
    /// 2. `candidates`: vertices that could extend the current clique
    /// 3. `excluded`: vertices already processed that can't be in the current clique
    ///
    /// The algorithm works as follows:
    /// 1. Base case: When both candidates and excluded sets are empty, we've found a maximal clique.
    ///    If it's larger than our previous best, update max_clique.
    ///
    /// 2. Pivot selection (optimization):
    ///    - Choose a vertex (pivot) from candidates ∪ excluded that connects to the most candidates
    ///    - This helps skip branches that won't lead to maximal cliques
    ///    - Only process candidates not connected to the pivot
    ///
    /// 3. For each candidate v not connected to the pivot:
    ///    a. Remove v from candidates and add it to the current clique
    ///    b. Create new candidate set: vertices in old candidates that connect to v
    ///    c. Create new excluded set: vertices in old excluded that connect to v
    ///    d. Recurse with updated sets
    ///    e. Remove v from clique and add to excluded
    ///
    /// ## Arguments
    /// * `clique` - Current clique being built
    /// * `candidates` - Candidate vertices that could extend the clique
    /// * `excluded` - Vertices that have already been processed
    /// * `max_clique` - Reference to store the maximum clique found so far
    fn bron_kerbosch(
        &self,
        clique: &mut HashSet<String>,
        candidates: &mut HashSet<String>,
        excluded: &mut HashSet<String>,
        max_clique: &mut HashSet<String>,
    ) {
        if candidates.is_empty() && excluded.is_empty() {
            if clique.len() > max_clique.len() {
                max_clique.clear();
                max_clique.extend(clique.iter().cloned());
            }

            return;
        }

        // Choose pivot vertex that maximizes the number of edges to candidates
        let pivot = candidates
            .iter()
            .chain(excluded.iter())
            .max_by_key(|&v| {
                candidates
                    .iter()
                    .filter(|&u| self.adj_list[v].contains(u))
                    .count()
            })
            .unwrap()
            .clone();

        // Process vertices not connected to pivot
        let candidates_copy = candidates.clone();
        for v in candidates_copy.iter() {
            if !self.adj_list[&pivot].contains(v) {
                candidates.remove(v);
                clique.insert(v.clone());

                let mut new_candidates: HashSet<_> = candidates
                    .iter()
                    .filter(|&u| self.adj_list[v].contains(u))
                    .cloned()
                    .collect();

                let mut new_excluded: HashSet<_> = excluded
                    .iter()
                    .filter(|&u| self.adj_list[v].contains(u))
                    .cloned()
                    .collect();

                self.bron_kerbosch(clique, &mut new_candidates, &mut new_excluded, max_clique);

                clique.remove(v);
                excluded.insert(v.clone());
            }
        }
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

    let graph: Graph = INPUT.parse().unwrap();
    let max_clique = graph.find_maximum_clique().join(",");

    let duration = start.elapsed();

    println!("Result: {max_clique}");
    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    response_part_2();
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
    fn test_graph_construction() {
        let graph: Graph = "a-b\nb-c\nc-a".parse().unwrap();

        assert!(graph.adj_list["a"].contains("b"));
        assert!(graph.adj_list["b"].contains("c"));
        assert!(graph.adj_list["c"].contains("a"));
    }

    #[test]
    fn test_solve_part_1() {
        let graph: Graph = TEST_INPUT.parse().unwrap();
        let result = graph.find_triads_with_t().len();

        assert_eq!(result, 7);
    }

    #[test]
    fn test_small_clique() {
        let graph: Graph = "a-b\nb-c\nc-a\na-c".parse().unwrap();

        assert_eq!(graph.find_maximum_clique(), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_solve_part_2() {
        let graph: Graph = TEST_INPUT.parse().unwrap();
        let result = graph.find_maximum_clique().join(",");

        assert_eq!(result, "co,de,ka,ta");
    }
}
