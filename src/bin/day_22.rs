///
/// # Day 22: Stock Market Prediction
///
/// ## Problem Overview
/// This solution addresses the Advent of Code 2024 Day 22 challenge which involves:
/// 1. Implementing a custom pseudo-random number generator (PRNG) to simulate stock prices
/// 2. Part 1: Calculating the sum of all PRNG states after 2000 iterations
/// 3. Part 2: Finding the sequence of price changes that yields the maximum return on investment
///
/// ## Implementation Details
/// The solution is structured around two main components:
/// 1. PseugoRandomNumberGenerator: Handles the generation of pseudo-random numbers
/// 2. Buyer: Manages stock price tracking and return on investment calculations
// Imports  ==============================================================================  Imports
use rustc_hash::FxHashMap; // Fast hashmap for better performance

// Variables  =========================================================================== Variables
const INPUT: &str = include_str!("../../data/inputs/day_22.txt");

///
/// # `PseugoRandomNumberGenerator`
/// A pseudo random number generator.
///
/// ## Fields
/// * `secret` - The secret number
#[derive(Debug, Copy, Clone)]
struct PseugoRandomNumberGenerator {
    secret: usize,
}

impl PseugoRandomNumberGenerator {
    const fn new(secret: usize) -> Self {
        Self { secret }
    }

    ///
    /// # `mix`
    /// Mixes the secret with a given number.
    /// Mixing is done by XORing the secret with the given number.
    ///
    /// ## Arguments
    /// * `a` - The number to mix with the secret
    fn mix(&mut self, a: usize) {
        self.secret ^= a;
    }

    ///
    /// # `prune`
    /// Prunes the secret by taking the modulo of the secret with 16777216.
    fn prune(&mut self) {
        self.secret %= 16777216;
    }

    ///
    /// # `next`
    /// Generates the next number in the sequence.
    ///
    /// This follows the instructions given in the problem statement.
    fn next(&mut self) {
        let mut temp = self.secret;
        self.mix(temp * 64);
        self.prune();

        temp = self.secret;
        self.mix(temp / 32);
        self.prune();

        temp = self.secret;
        self.mix(temp * 2048);
        self.prune();
    }
}

///
/// # `Buyer`
/// A buyer that buys stocks.
/// The buyer uses a pseudo random number generator to generate stock prices.
///
/// ## Fields
/// * `prng` - The pseudo random number generator
/// * `prices` - The last 4 stock prices
/// * `changes` - The changes in stock prices
/// * `roi` - The return on investment for the last 4 stock price changes
#[derive(Debug, Clone)]
struct Buyer {
    prng: PseugoRandomNumberGenerator,
    prices: [usize; 4],
    changes: [isize; 4],
    roi: FxHashMap<[isize; 4], usize>,
}

impl Buyer {
    fn new(init: usize) -> Self {
        let mut buyer = Self {
            prng: PseugoRandomNumberGenerator::new(init),
            prices: [0; 4],
            changes: [0; 4],
            roi: FxHashMap::default(),
        };

        buyer.prices[0] = buyer.prng.secret % 10;
        buyer.populate();

        buyer
    }

    ///
    /// # `populate`
    /// Populates the stock prices and changes.
    ///
    /// Since we need the 2000th stock price, we populate the stock prices and changes for the first 2000 stock prices.
    /// We also calculate the return on investment for the last 4 stock price changes.
    ///
    /// The return on investment is calculated as follows:
    /// * If the stock price changes are [a, b, c, d] and the stock price at the end is x, then the return on investment is x.
    /// * We store the return on investment for the last 4 stock price changes in a hashmap.
    /// * The key for the hashmap is the last 4 stock price changes and the value is the stock price at the end.
    /// * This way, we can easily lookup the stock price at the end for the last 4 stock price changes.
    /// * We use this to calculate the 2000th stock price.
    /// * We also use this to calculate the maximum return on investment.
    fn populate(&mut self) {
        for i in 1..2001 {
            self.prng.next();
            self.prices[i % 4] = self.prng.secret % 10;

            let change = self.prices[i % 4] as isize - self.prices[(i - 1) % 4] as isize;

            self.changes[i % 4] = change;

            if i >= 4 {
                let last_seq = [
                    self.changes[(i - 3) % 4],
                    self.changes[(i - 2) % 4],
                    self.changes[(i - 1) % 4],
                    self.changes[i % 4],
                ];

                self.roi.entry(last_seq).or_insert(self.prices[i % 4]);
            }
        }
    }
}
// Functions  =========================================================================== Functions
pub fn response_part_1() {
    println!("Day 22 - Part 1");
    let start = std::time::Instant::now();

    let sum: usize = INPUT
        .trim()
        .lines()
        .map(|line| {
            let init_prng = PseugoRandomNumberGenerator::new(line.parse().unwrap());
            let prng = (0..2000).fold(init_prng, |mut prng, _| {
                prng.next();
                prng
            });
            prng.secret
        })
        .sum();

    let duration = start.elapsed();

    println!("Sum: {}", sum);
    println!("Duration: {duration:?}");
}

pub fn response_part_2() {
    println!("Day 22 - Part 2");
    let start = std::time::Instant::now();

    let mut sequence_sums = FxHashMap::default();
    INPUT
        .trim()
        .lines()
        .map(|buyer_init| Buyer::new(buyer_init.parse().unwrap()).roi)
        .for_each(|buyer| {
            buyer.iter().for_each(|(&sequence, &value)| {
                sequence_sums
                    .entry(sequence)
                    .and_modify(|e| *e += value)
                    .or_insert(value);
            });
        });

    let most_bananas = *sequence_sums.values().max().unwrap();

    let duration = start.elapsed();

    println!("Most bananas: {}", most_bananas);
    println!("Duration: {duration:?}");
}

fn main() {
    response_part_1();
    response_part_2();
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_part_1() {
        let sum: usize = "\
1
10
100
2024"
            .trim()
            .lines()
            .map(|line| {
                let init_prng = PseugoRandomNumberGenerator::new(line.parse().unwrap());
                let prng = (0..2000).fold(init_prng, |mut prng, _| {
                    prng.next();
                    prng
                });
                prng.secret
            })
            .sum();

        assert_eq!(sum, 37327623);
    }

    #[test]
    fn test_example_input_part_2() {
        let mut sequence_sums = FxHashMap::default();

        "\
1
2
3
2024"
            .trim()
            .lines()
            .map(|buyer_init| Buyer::new(buyer_init.parse().unwrap()).roi)
            .for_each(|buyer| {
                buyer.iter().for_each(|(&sequence, &value)| {
                    sequence_sums
                        .entry(sequence)
                        .and_modify(|e| *e += value)
                        .or_insert(value);
                });
            });

        let most_bananas = *sequence_sums.values().max().unwrap();

        assert_eq!(most_bananas, 23);
    }

    #[test]
    fn test_prng_initialization() {
        let prng = PseugoRandomNumberGenerator::new(12345);
        assert_eq!(prng.secret, 12345);
    }

    #[test]
    fn test_prng_mix() {
        let mut prng = PseugoRandomNumberGenerator::new(12345);
        prng.mix(64);
        assert_eq!(prng.secret, 12345 ^ 64);
    }

    #[test]
    fn test_prng_prune() {
        let mut prng = PseugoRandomNumberGenerator::new(16777217); // One more than the modulo
        prng.prune();
        assert_eq!(prng.secret, 1);
    }

    #[test]
    fn test_prng_next_sequence() {
        let mut prng = PseugoRandomNumberGenerator::new(12345);
        let initial = prng.secret;
        prng.next();
        assert_ne!(prng.secret, initial, "PRNG should change after next()");
        assert!(prng.secret < 16777216, "PRNG should stay within bounds");
    }

    #[test]
    fn test_buyer_initialization() {
        let buyer = Buyer::new(12345);
        assert_eq!(buyer.prices.len(), 4);
        assert_eq!(buyer.changes.len(), 4);
        assert!(!buyer.roi.is_empty(), "ROI map should be populated");
    }

    #[test]
    fn test_buyer_price_changes() {
        let buyer = Buyer::new(12345);
        // Verify that changes array contains valid price differences
        for change in buyer.changes.iter() {
            assert!(
                *change >= -9 && *change <= 9,
                "Price changes should be between -9 and 9 since prices are mod 10"
            );
        }
    }

    #[test]
    fn test_buyer_roi_patterns() {
        let buyer = Buyer::new(12345);
        for (pattern, &price) in buyer.roi.iter() {
            assert_eq!(pattern.len(), 4, "Pattern should contain 4 changes");
            assert!(price < 10, "Final price should be less than 10");
        }
    }
}
