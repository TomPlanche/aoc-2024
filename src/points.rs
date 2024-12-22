use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

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

#[derive(Debug, Clone, Copy, Hash)]
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

impl<T: Number> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Point { x, y }
    }
}

impl<T: Number> From<&(T, T)> for Point<T> {
    fn from(&(x, y): &(T, T)) -> Self {
        Point { x, y }
    }
}

impl<T: Number> From<[T; 2]> for Point<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Point { x, y }
    }
}

impl<T: Number> From<Point<T>> for (T, T) {
    fn from(point: Point<T>) -> (T, T) {
        (point.x, point.y)
    }
}

impl<T> FromStr for Point<T>
where
    T: Number + FromStr,
    T::Err: std::fmt::Debug,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() < 2 {
            return Err("Invalid point format".to_string());
        }

        let x = parts[0]
            .parse()
            .map_err(|e| format!("Failed to parse x coordinate: {:?}", e))?;
        let y = parts[1]
            .parse()
            .map_err(|e| format!("Failed to parse y coordinate: {:?}", e))?;

        Ok(Point { x, y })
    }
}

impl<T: Number> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Number> Eq for Point<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let p = Point::new(1, 2);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);

        // Test with different numeric types
        let p_f32 = Point::new(1.0f32, 2.0f32);
        let p_i64 = Point::new(1i64, 2i64);
        assert_eq!(p_f32.x, 1.0f32);
        assert_eq!(p_i64.x, 1i64);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let result = p1 + p2;
        assert_eq!(result, Point::new(4, 6));

        // Test with negative numbers
        let p3 = Point::new(-1, -2);
        let p4 = Point::new(1, 2);
        assert_eq!(p3 + p4, Point::new(0, 0));

        // Test with floating point numbers
        let p5 = Point::new(1.5f64, 2.5f64);
        let p6 = Point::new(1.5f64, 2.5f64);
        assert_eq!(p5 + p6, Point::new(3.0f64, 5.0f64));
    }

    #[test]
    fn test_point_sub() {
        let p1 = Point::new(3, 4);
        let p2 = Point::new(1, 2);
        let result = p1 - p2;
        assert_eq!(result, Point::new(2, 2));

        // Test with negative numbers
        let p3 = Point::new(-1, -2);
        let p4 = Point::new(1, 2);
        assert_eq!(p3 - p4, Point::new(-2, -4));

        // Test with floating point numbers
        let p5 = Point::new(3.5f64, 4.5f64);
        let p6 = Point::new(1.5f64, 2.5f64);
        assert_eq!(p5 - p6, Point::new(2.0f64, 2.0f64));
    }

    #[test]
    fn test_point_display() {
        let p = Point::new(1, 2);
        assert_eq!(format!("{}", p), "(1, 2)");

        let p_float = Point::new(1.5f64, 2.5f64);
        assert_eq!(format!("{}", p_float), "(1.5, 2.5)");
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(4, 5);
        assert_eq!(p1.manhattan_distance(&p2), 7);

        // Test with negative numbers
        let p3 = Point::new(-1, -1);
        let p4 = Point::new(2, 2);
        assert_eq!(p3.manhattan_distance(&p4), 6);

        // Test with floating point numbers
        let p5 = Point::new(1.0f64, 1.0f64);
        let p6 = Point::new(4.0f64, 5.0f64);
        assert_eq!(p5.manhattan_distance(&p6), 7.0f64);

        // Test with zero distance
        let p7 = Point::new(1, 1);
        let p8 = Point::new(1, 1);
        assert_eq!(p7.manhattan_distance(&p8), 0);
    }

    #[test]
    fn test_from_tuple() {
        // Test From<(T, T)>
        let tuple = (1, 2);
        let point: Point<i32> = Point::from(tuple);
        assert_eq!(point, Point::new(1, 2));

        // Test From<&(T, T)>
        let tuple_ref = &(3, 4);
        let point: Point<i32> = Point::from(tuple_ref);
        assert_eq!(point, Point::new(3, 4));

        // Test with different numeric types
        let float_tuple = (1.5f64, 2.5f64);
        let point: Point<f64> = Point::from(float_tuple);
        assert_eq!(point, Point::new(1.5f64, 2.5f64));
    }

    #[test]
    fn test_point_equality() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(1, 2);
        let p3 = Point::new(2, 1);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);

        // Test with different numeric types
        let p4 = Point::new(1.0f64, 2.0f64);
        let p5 = Point::new(1.0f64, 2.0f64);
        assert_eq!(p4, p5);
    }

    #[test]
    fn test_point_copy_clone() {
        let p1 = Point::new(1, 2);
        let p2 = p1; // Copy
        assert_eq!(p1, p2);

        let p3 = p1.clone(); // Clone
        assert_eq!(p1, p3);
    }

    #[test]
    fn test_point_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Point::new(1, 2));
        set.insert(Point::new(1, 2)); // Duplicate
        set.insert(Point::new(2, 1));

        assert_eq!(set.len(), 2); // Should only contain unique points
    }
}
