use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    North,
    NorthEst,
    Est,
    SouthEst,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn modifier(&self) -> (i32, i32) {
        match self {
            Direction::NorthWest => (-1, -1),
            Direction::North => (0, -1),
            Direction::NorthEst => (1, -1),
            Direction::Est => (1, 0),
            Direction::SouthEst => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
        }
    }
}
