use std::fmt;
use std::ops::{Add, Sub};

// Define a trait that combines the necessary numeric traits
pub trait Number:
    Copy + PartialOrd + Add<Output = Self> + Sub<Output = Self> + fmt::Display
{
}

// Implement the Number trait for the built-in numeric types
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for f32 {}
impl Number for f64 {}
impl Number for usize {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T: Number> {
    pub x: T,
    pub y: T,
}

// Most of the solutions will implement their own `FromStr` trait for the `Point` struct.
#[allow(dead_code)]
impl<T: Number> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: Number> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Number> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Number> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[allow(dead_code)]
impl<T: Number> Point<T> {
    pub fn manhattan_distance(&self, other: &Self) -> T {
        let dx = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        dx + dy
    }
}
