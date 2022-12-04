#[derive(Debug, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::RIGHT
        }
    }
}
