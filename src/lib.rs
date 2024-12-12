mod directions;
mod points;

pub use directions::Direction;
pub use points::Point;

///
/// # gcd
/// Greatest common divisor of two numbers
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
