use crate::player::{Player, Position};

struct GameArea(u8, u8);

pub struct GameState {
    player: Player,
    game_area: GameArea,
    powerup: Position,
}
pub enum Tile {
    Normal,
    Tail,
    Wall,
    Powerup,
}

enum GameCell {
    Head,
    Tail,
    Powerup,
    Empty,
}

fn generate_game_visual(game_state: GameState) {
    todo!();
}

impl GameState {
    pub fn new() -> Self {
        let player = Player::new();
        let game_area = GameArea(80, 30);
        let powerup = Position { x: 10, y: 10 };
        return Self {
            player,
            game_area,
            powerup,
        };
    }
    pub fn run(&mut self) {
        while let Some(_) = self.tick() {
            // handle input
            self.render();
        }
    }
    fn tick(&mut self) -> Option<()> {
        let new_position = self.player.calculate_new_position();
        match self.check_position(&new_position) {
            Tile::Wall | Tile::Tail => None,
            Tile::Normal => {
                self.player
                    .move_player(new_position, crate::player::MoveType::Normal);
                Some(())
            }
            Tile::Powerup => {
                self.player
                    .move_player(new_position, crate::player::MoveType::PowerUp);
                Some(())
            }
        }
    }
    fn check_position(&self, new_position: &Position) -> Tile {
        todo!();
    }
    fn determine_cell(&self, row_number: usize, column_number: usize) -> GameCell {
        todo!()
    }
    fn render(&self) {
        let mut display = String::new();
        let horizontal_border =
            "--------------------------------------------------------------------------------";
        display.push_str(&horizontal_border);
        for row_number in 1..30 {
            let mut row = String::new();
            row.push_str("|");
            for column_number in 1..80 {
                match self.determine_cell(row_number, column_number) {
                    GameCell::Head => {
                        row.push('S');
                    }
                    GameCell::Tail => {
                        row.push('O');
                    }
                    GameCell::Powerup => {
                        row.push('#');
                    }
                    GameCell::Empty => {
                        row.push(' ');
                    }
                }
            }
            row.push_str("|");
            display.push_str(&row);
        }
        display.push_str(&horizontal_border);
        println!("{}", display);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
