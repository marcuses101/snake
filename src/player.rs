use std::{collections::VecDeque, fmt};

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
    pub column_number: isize,
    pub row_number: isize,
}
impl Position {
    fn new(column_number: isize, row_number: isize) -> Self {
        return Self {
            column_number,
            row_number,
        };
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "column_number: {}, row_number: {}",
            self.column_number, self.row_number
        )
    }
}
#[derive(Debug)]
pub struct Tail {
    pub positions: VecDeque<Position>,
}

impl Tail {
    pub fn new(column_number: isize, row_number: isize) -> Self {
        return Tail {
            positions: VecDeque::from([Position::new(column_number, row_number)]),
        };
    }
    pub fn check(&self, column_number: isize, row_number: isize) -> bool {
        return self
            .positions
            .iter()
            .any(|pos| pos.column_number == column_number && pos.row_number == row_number)
            .to_owned();
    }
}

#[derive(Debug)]
pub struct Player {
    pub head_position: Position,
    pub heading: Direction,
}

impl Player {
    pub fn new(x: isize, y: isize) -> Self {
        return Self {
            head_position: Position {
                column_number: x,
                row_number: y,
            },
            heading: Direction::Right,
        };
    }
    /// Changes the head_position of the player and returns the previous position
    pub fn move_player(&mut self, new_position: Position) -> Position {
        let previous_position = self.head_position;
        self.head_position = new_position;
        return previous_position;
    }
    pub fn calculate_new_position(&self) -> Position {
        match self.heading {
            Direction::Up => Position {
                column_number: self.head_position.column_number,
                row_number: self.head_position.row_number - 1,
            },
            Direction::Right => Position {
                column_number: self.head_position.column_number + 1,
                row_number: self.head_position.row_number,
            },
            Direction::Left => Position {
                column_number: self.head_position.column_number - 1,
                row_number: self.head_position.row_number,
            },
            Direction::Down => Position {
                column_number: self.head_position.column_number,
                row_number: self.head_position.row_number + 1,
            },
        }
    }
    pub fn change_heading(&mut self, new_direction: Direction) {
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
