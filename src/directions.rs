use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug)]
pub enum DirectionError {
    InvalidDirection,
}

// direction to (y, x)
impl From<Direction> for (isize, isize) {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => (-1, 0),       // Changed from (0, 1)
            Direction::Down => (1, 0),      // Changed from (0, -1)
            Direction::Left => (0, -1),     // Changed from (-1, 0)
            Direction::Right => (0, 1),     // Changed from (1, 0)
            Direction::UpLeft => (-1, -1),  // Changed from (-1, 1)
            Direction::UpRight => (-1, 1),  // Changed from (1, 1)
            Direction::DownLeft => (1, -1), // Changed from (-1, -1)
            Direction::DownRight => (1, 1), // Changed from (1, -1)
        }
    }
}

// (y, x) + direction
impl Add<Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, d: Direction) -> Self::Output {
        let (dy, dx) = d.into(); // Changed from (dx, dy)
        (self.0 + dy, self.1 + dx) // Changed from (self.0 + dx, self.1 + dy)
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, d: Direction) -> Self::Output {
        let (dy, dx) = d.into(); // Changed from (dx, dy)

        // wrapping_add to avoid panics
        (
            self.0.wrapping_add(dy as usize), // Changed order
            self.1.wrapping_add(dx as usize),
        )
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            '<' => Self::Left,
            'v' => Self::Down,
            _ => panic!(),
        }
    }
}

impl Direction {
    pub fn row_delta(&self) -> isize {
        match self {
            Direction::Up | Direction::UpLeft | Direction::UpRight => -1,
            Direction::Down | Direction::DownLeft | Direction::DownRight => 1,
            _ => 0,
        }
    }

    pub fn col_delta(&self) -> isize {
        match self {
            Direction::Up | Direction::Down => 0,
            Direction::Left | Direction::UpLeft | Direction::DownLeft => -1,
            Direction::Right | Direction::UpRight | Direction::DownRight => 1,
        }
    }

    pub fn from_points(
        from: (isize, isize),
        to: (isize, isize),
    ) -> Result<Direction, DirectionError> {
        let dy = to.0 - from.0; // Changed from dx
        let dx = to.1 - from.1; // Changed from dy

        // Normalize the deltas to -1, 0, or 1
        let dy = dy.signum(); // Changed from dx
        let dx = dx.signum(); // Changed from dy

        match (dy, dx) {
            // Changed from (dx, dy)
            (-1, 0) => Ok(Direction::Up),
            (1, 0) => Ok(Direction::Down),
            (0, -1) => Ok(Direction::Left),
            (0, 1) => Ok(Direction::Right),
            (-1, -1) => Ok(Direction::UpLeft),
            (-1, 1) => Ok(Direction::UpRight),
            (1, -1) => Ok(Direction::DownLeft),
            (1, 1) => Ok(Direction::DownRight),
            (0, 0) => Err(DirectionError::InvalidDirection),
            _ => panic!("Invalid direction: ({dy}, {dx})"),
        }
    }

    pub fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn turn_counterclockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn move_forward(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => Some((pos.0.checked_sub(1)?, pos.1)),
            Direction::Down => Some((pos.0 + 1, pos.1)),
            Direction::Right => Some((pos.0, pos.1 + 1)),
            Direction::Left => Some((pos.0, pos.1.checked_sub(1)?)),
            _ => None,
        }
    }
}
