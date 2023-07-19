use std::{collections::VecDeque, fmt, vec};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum MoveType {
    Normal,
    Powerup,
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

pub struct PowerUp {
    position: Position,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    x: isize,
    y: isize,
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
    fn update_positions(&mut self, new_position: Position, new_position_tile: MoveType) -> () {
        self.positions.push_front(new_position);
        if let MoveType::Normal = new_position_tile {
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
    pub fn move_player(&mut self) -> () {
        let old_position = self.head_position;
        self.head_position = match self.heading {
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
        };
        self.tail.update_positions(old_position, MoveType::Normal);
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
