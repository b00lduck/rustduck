#[derive(PartialEq,Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
    Blocked
}

impl Direction {
    pub fn to_vector(&self) -> Vector2 {
        match self {
            Direction::North => Vector2 { x: 0, y: -1 },
            Direction::South => Vector2 { x: 0, y: 1 },
            Direction::West => Vector2 { x: -1, y: 0 },
            Direction::East => Vector2 { x: 1, y: 0 },
            Direction::Blocked => Vector2 { x: 0, y: 0 },
        }
    }
    pub fn turn_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
            Direction::Blocked => Direction::Blocked
        }
    }
    pub fn turn_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
            Direction::Blocked => Direction::Blocked 
        }
    }
    pub fn clone(&self) -> Direction {
        match self {
            Direction::North => Direction::North,
            Direction::South => Direction::South,
            Direction::West => Direction::West,
            Direction::East => Direction::East,
            Direction::Blocked => Direction::Blocked
        }
    }    
}

pub struct Vector2 {
    pub x: i16,
    pub y: i16
}