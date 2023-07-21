use std::{collections::VecDeque, fmt};

use crate::game::Tile;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
pub enum MoveType {
    Normal,
    PowerUp,
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

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}
impl Position {
    fn new(x: isize, y: isize) -> Self {
        return Self { x, y };
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
#[derive(Debug)]
pub struct Tail {
    positions: VecDeque<Position>,
}

impl Tail {
    fn update_positions(&mut self, new_position: Position, move_type: MoveType) -> () {
        self.positions.push_front(new_position);
        if let MoveType::Normal = move_type {
            self.positions.pop_back();
        }
    }
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
                positions: VecDeque::from([Position::new(-1, 0)]),
            },
        };
    }
    pub fn move_player(&mut self, new_position: Position, move_type: MoveType) {
        todo!();
    }
    pub fn calculate_new_position(&self) -> Position {
        match self.heading {
            Direction::Up => Position {
                x: self.head_position.x,
                y: self.head_position.y + 1,
            },
            Direction::Right => Position {
                x: self.head_position.x + 1,
                y: self.head_position.y,
            },
            Direction::Left => Position {
                x: self.head_position.x - 1,
                y: self.head_position.y,
            },
            Direction::Down => Position {
                x: self.head_position.x,
                y: self.head_position.y - 1,
            },
        }
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
}
