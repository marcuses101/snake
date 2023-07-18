use std::{fmt, vec};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<String> for Direction {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "l" => Ok(Self::Left),
            "r" => Ok(Self::Right),
            "u" => Ok(Self::Up),
            "d" => Ok(Self::Down),
            _ => Err(r#"Only "l", "r", "u", and "d" permitted "#),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    x: isize,
    y: isize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
#[derive(Debug)]
pub struct Tail {
    positions: Vec<Position>,
    length: usize,
}

#[derive(Debug)]
pub struct Player {
    pub head_position: Position,
    pub heading: Direction,
    pub tail: Tail,
}

impl Player {
    pub fn new() -> Self {
        return Self {
            head_position: Position { x: 0, y: 0 },
            heading: Direction::Right,
            tail: Tail {
                positions: vec![Position { x: -1, y: 0 }],
                length: 1,
            },
        };
    }
    pub fn move_player(&mut self) -> () {
        match self.heading {
            Direction::Up => self.head_position.y += 1,
            Direction::Right => self.head_position.x += 1,
            Direction::Down => self.head_position.y -= 1,
            Direction::Left => self.head_position.x -= 1,
        };
    }
    pub fn change_heading(&mut self, new_direction: Direction) -> () {
        let new_heading: Option<Direction> = match (&self.heading, new_direction) {
            (Direction::Up | Direction::Down, Direction::Right) => Some(Direction::Right),
            (Direction::Up | Direction::Down, Direction::Left) => Some(Direction::Left),
            (Direction::Left | Direction::Right, Direction::Up) => Some(Direction::Up),
            (Direction::Left | Direction::Right, Direction::Down) => Some(Direction::Down),
            _ => None,
        };
        if let Some(direction) = new_heading {
            self.heading = direction;
        }
    }
    pub fn grow() {
        todo!();
    }
}
