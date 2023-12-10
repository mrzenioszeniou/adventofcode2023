use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(from: char) -> Result<Self, Self::Error> {
        match from {
            'N' | '^' => Ok(Self::North),
            'S' | 'v' => Ok(Self::South),
            'W' | '<' => Ok(Self::West),
            'E' | '>' => Ok(Self::East),
            _ => Err(format!("`{}` is not a valid as direction", from)),
        }
    }
}

impl From<Direction> for char {
    fn from(from: Direction) -> Self {
        match from {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn u_turn(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn forward_step(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }
}

pub fn neighbours(pos: (isize, isize)) -> Vec<((isize, isize), Direction)> {
    [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .map(|dir| {
        let step = dir.forward_step();

        ((pos.0 + step.0, pos.1 + step.1), *dir)
    })
    .collect()
}

/// ```
/// o--pos.0-->
/// |
/// pos.1
/// |
/// ⌄
/// ```
pub fn step(pos: (isize, isize), dir: Direction) -> (isize, isize) {
    let step = dir.forward_step();
    (pos.0 + step.0, pos.1 + step.1)
}
