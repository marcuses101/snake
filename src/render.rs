use std::fmt::Display;

use crate::game::{GameBoard, GameCell, Wall};

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: String = self.0.rows_iter().fold(String::new(), |mut acc, row| {
            row.for_each(|tile| {
                let tile_char = match tile {
                    GameCell::Tail => 'O',
                    GameCell::Powerup => '*',
                    GameCell::Empty => ' ',
                    GameCell::Head(head_direction) => match head_direction {
                        crate::player::Direction::Up => 'Ʌ',
                        crate::player::Direction::Right => '>',
                        crate::player::Direction::Down => 'V',
                        crate::player::Direction::Left => '<',
                    },
                    GameCell::Edge(wall) => match wall {
                        Wall::Horizontal => '═',
                        Wall::Vertical => '║',
                        Wall::TopLeft => '╔',
                        Wall::TopRight => '╗',
                        Wall::BottomLeft => '╚',
                        Wall::BottomRight => '╝',
                    },
                };
                acc.push(tile_char);
            });
            acc.push_str("\n\r");
            acc
        });
        write!(f, "{}", output)
    }
}
