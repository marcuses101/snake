use std::fmt;

#[derive(Debug)]
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
pub struct Player {
    pub position: Position,
    pub direction: Direction,
    pub length: isize,
}

impl Player {
    pub fn new() -> Self {
        return Self {
            position: Position { x: 0, y: 0 },
            direction: Direction::Right,
            length: 1,
        };
    }
    pub fn move_player(&mut self) -> () {
        match self.direction {
            Direction::Up => self.position.y += 1,
            Direction::Right => self.position.x += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
        };
    }
    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}
