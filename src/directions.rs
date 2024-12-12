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

// direction to (usize, usize)
impl From<Direction> for (isize, isize) {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, 1),
            Direction::UpRight => (1, 1),
            Direction::DownLeft => (-1, -1),
            Direction::DownRight => (1, -1),
        }
    }
}

impl Direction {
    pub fn row_delta(&self) -> isize {
        match self {
            Direction::Up => 0,
            Direction::Down => 0,
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::UpLeft => -1,
            Direction::UpRight => 1,
            Direction::DownLeft => -1,
            Direction::DownRight => 1,
        }
    }

    pub fn col_delta(&self) -> isize {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
            Direction::Left => 0,
            Direction::Right => 0,
            Direction::UpLeft => 1,
            Direction::UpRight => 1,
            Direction::DownLeft => -1,
            Direction::DownRight => -1,
        }
    }
}
