use std::fmt::Display;

use crate::game::{GameBoard, GameCell};

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: String = self.0.rows_iter().fold(String::new(), |mut acc, row| {
            row.for_each(|tile| {
                let tile_char = match tile {
                    GameCell::Head => 'S',
                    GameCell::Tail => 'O',
                    GameCell::Powerup => '*',
                    GameCell::Empty => ' ',
                    GameCell::Edge => '+',
                };
                acc.push(tile_char);
            });
            acc.push_str("\n\r");
            acc
        });
        write!(f, "{}", output)
    }
}
