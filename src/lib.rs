mod directions;
mod points;

pub use directions::Direction;
pub use points::Point;

///
/// # gcd
/// Greatest common divisor of two numbers
///
/// ## Arguments
/// * `a` - The first number
/// * `b` - The second number
///
/// ## Returns
/// * `T` - The greatest common divisor of `a` and `b`
pub fn gcd<T>(a: T, b: T) -> T
where
    T: std::ops::Rem<Output = T> + PartialEq + Copy + num::Zero,
{
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

///
/// # `calculate_variance`
/// Calculate the variance of the x-coordinates of a list of positions.
/// The variance is a measure of how spread out the values are.
///
/// # Arguments
/// * `positions` - A slice of tuples where each tuple represents a position (x, y)
///
/// # Returns
/// * `f64` - The variance of the x-coordinates
pub fn calculate_variance(positions: &[(i32, i32)]) -> f64 {
    // Calculate the mean of the x-coordinates
    let mean = positions.iter().map(|&(x, _)| f64::from(x)).sum::<f64>() / positions.len() as f64;

    // Calculate the variance of the x-coordinates
    let variance = positions
        .iter()
        .map(|&(x, _)| {
            let diff = f64::from(x) - mean;

            diff * diff
        })
        .sum::<f64>()
        / positions.len() as f64;

    variance
}
