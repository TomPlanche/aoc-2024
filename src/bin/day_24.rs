///
/// # Circuit Simulator (Day 24)
/// Implements a boolean circuit simulator that evaluates logic gates and propagates
/// signals through the circuit. Supports XOR, AND, and OR operations.
///
// Imports ================================================================================ Imports
use std::collections::HashMap;
use std::str::FromStr;

// Constants ============================================================================ Constants
const INPUT: &str = include_str!("../../data/inputs/day_24.txt");

// Types ==================================================================================== Types
/// Custom error type for circuit parsing and evaluation
#[derive(Debug)]
pub enum CircuitError {
    ParseError(String),
    EvaluationError(String),
}

/// Represents the different types of logic gates in the circuit
#[derive(Debug, Clone)]
enum Gate {
    Xor(String, String, String), // input1, input2, output
    And(String, String, String), // input1, input2, output
    Or(String, String, String),  // input1, input2, output
}

impl Gate {
    ///
    /// # `evaluate`
    /// Evaluates this gate given the current circuit state
    ///
    /// ## Arguments
    /// * `state` - Current state of all signals in the circuit
    ///
    /// ## Returns
    /// * `Option<(String, bool)>` - The output signal and its new value, if inputs are available
    fn evaluate(&self, state: &HashMap<String, bool>) -> Option<(String, bool)> {
        match self {
            Gate::Xor(in1, in2, out) => {
                if let (Some(&v1), Some(&v2)) = (state.get(in1), state.get(in2)) {
                    Some((out.clone(), v1 ^ v2))
                } else {
                    None
                }
            }
            Gate::And(in1, in2, out) => {
                if let (Some(&v1), Some(&v2)) = (state.get(in1), state.get(in2)) {
                    Some((out.clone(), v1 & v2))
                } else {
                    None
                }
            }
            Gate::Or(in1, in2, out) => {
                if let (Some(&v1), Some(&v2)) = (state.get(in1), state.get(in2)) {
                    Some((out.clone(), v1 | v2))
                } else {
                    None
                }
            }
        }
    }
}

/// Circuit state holding input values and gate definitions
#[derive(Debug)]
struct Circuit {
    inputs: HashMap<String, bool>,
    gates: Vec<Gate>,
}

impl FromStr for Circuit {
    type Err = CircuitError;

    ///
    /// # `from_str`
    /// Parses a circuit definition from string input
    ///
    /// ## Format
    /// Input should contain lines of two types:
    /// 1. Input definitions: "x00: 1" or "y00: 0"
    /// 2. Gate definitions: "a XOR b -> c" or "d AND e -> f" or "g OR h -> i"
    ///
    /// ## Arguments
    /// * `s` - Input string containing circuit definition
    ///
    /// ## Returns
    /// * `Result<Circuit, CircuitError>` - Parsed circuit or error
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs = HashMap::new();
        let mut gates = Vec::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse input definitions
            if line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    return Err(CircuitError::ParseError(format!(
                        "Invalid input format: {}",
                        line
                    )));
                }

                let signal_name = parts[0].trim().to_string();
                let value = parts[1]
                    .trim()
                    .parse::<i32>()
                    .map_err(|_| CircuitError::ParseError(format!("Invalid value in: {}", line)))?;

                inputs.insert(signal_name, value == 1);
                continue;
            }

            // Parse gate definitions
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 5 || parts[3] != "->" {
                return Err(CircuitError::ParseError(format!(
                    "Invalid gate format: {}",
                    line
                )));
            }

            let gate = match parts[1] {
                "XOR" => Gate::Xor(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                ),
                "AND" => Gate::And(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                ),
                "OR" => Gate::Or(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                ),
                _ => {
                    return Err(CircuitError::ParseError(format!(
                        "Unknown gate type: {}",
                        parts[1]
                    )))
                }
            };
            gates.push(gate);
        }

        Ok(Circuit { inputs, gates })
    }
}

impl Circuit {
    ///
    /// # `evaluate`
    /// Evaluates the circuit and returns the final state of all signals
    ///
    /// ## Algorithm
    /// 1. Start with the initial input values
    /// 2. Repeatedly evaluate all gates until no more changes occur
    /// 3. Return the final state of all signals
    ///
    /// ## Returns
    /// * `Result<HashMap<String, bool>, CircuitError>` - Final state of all signals
    fn evaluate(&self) -> Result<HashMap<String, bool>, CircuitError> {
        let mut state = self.inputs.clone();
        let mut changed = true;

        // Keep evaluating gates until no more changes occur
        while changed {
            changed = false;
            for gate in &self.gates {
                if let Some((output, value)) = gate.evaluate(&state) {
                    if state.get(&output) != Some(&value) {
                        state.insert(output, value);
                        changed = true;
                    }
                }
            }
        }

        Ok(state)
    }

    ///
    /// # `get_output`
    /// Returns the value of a specific output signal
    ///
    /// ## Arguments
    /// * `output_name` - Name of the output signal to retrieve
    ///
    /// ## Returns
    /// * `Result<Option<bool>, CircuitError>` - Value of the output signal or error
    fn get_output(&self, output_name: &str) -> Result<Option<bool>, CircuitError> {
        let state = self.evaluate()?;
        Ok(state.get(output_name).copied())
    }
}

pub fn response_part_1() {
    println!("Day 24 - Part 1");
    let start = std::time::Instant::now();

    // Parse the circuit from input
    let circuit = Circuit::from_str(INPUT).unwrap();

    // Get all outputs z00 through z63 and combine them into a u64
    let mut result: u64 = 0;
    for i in 0..64 {
        let output_name = format!("z{:02}", i);
        if let Some(value) = circuit.get_output(&output_name).unwrap() {
            result |= (value as u64) << i;
        }
    }

    let duration = start.elapsed();

    println!("Result: {}", result);
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 24 - Part 2");
    let start = std::time::Instant::now();

    // Part 2 solution will go here...

    let duration = start.elapsed();
    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    //response_part_2();
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_circuit() {
        let input = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

        let circuit = Circuit::from_str(input).unwrap();

        // Test individual outputs
        assert_eq!(circuit.get_output("z00").unwrap(), Some(false)); // AND: 1 & 0 = 0
        assert_eq!(circuit.get_output("z01").unwrap(), Some(false)); // XOR: 1 ^ 1 = 0
        assert_eq!(circuit.get_output("z02").unwrap(), Some(true)); // OR: 1 | 0 = 1

        // Test full evaluation
        let state = circuit.evaluate().unwrap();
        assert_eq!(state.get("z00"), Some(&false));
        assert_eq!(state.get("z01"), Some(&false));
        assert_eq!(state.get("z02"), Some(&true));
    }

    #[test]
    fn test_chained_gates() {
        let input = "\
x00: 1
y00: 1

x00 AND y00 -> a01
a01 XOR y00 -> z00
a01 OR x00 -> z01";

        let circuit = Circuit::from_str(input).unwrap();
        let state = circuit.evaluate().unwrap();

        // First gate: AND(1,1) = 1 -> a01
        assert_eq!(state.get("a01"), Some(&true));
        // Second gate: XOR(1,1) = 0 -> z00
        assert_eq!(state.get("z00"), Some(&false));
        // Third gate: OR(1,1) = 1 -> z01
        assert_eq!(state.get("z01"), Some(&true));
    }

    #[test]
    fn test_parse_errors() {
        // Test invalid input format
        let invalid_input = "x00: invalid";
        assert!(matches!(
            Circuit::from_str(invalid_input),
            Err(CircuitError::ParseError(_))
        ));

        // Test invalid gate format
        let invalid_gate = "x00 INVALID y00 -> z00";
        assert!(matches!(
            Circuit::from_str(invalid_gate),
            Err(CircuitError::ParseError(_))
        ));
    }

    #[test]
    fn test_empty_input() {
        let empty = "";
        let circuit = Circuit::from_str(empty).unwrap();
        assert!(circuit.inputs.is_empty());
        assert!(circuit.gates.is_empty());
    }
}
