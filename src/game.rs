use crate::{
    player::{Direction, Player, Position, Tail},
    utils::read_input,
};

struct GameArea(u8, u8);

pub struct GameState {
    player: Player,
    tail: Tail,
    game_area: GameArea,
    powerup: Position,
}

enum GameCell {
    Head,
    Tail,
    Powerup,
    Empty,
    Edge,
}

fn determine_game_cell(
    game_state: &GameState,
    column_number: isize,
    row_number: isize,
) -> GameCell {
    if column_number == 0
        || row_number == 0
        || column_number >= game_state.game_area.0.into()
        || row_number >= game_state.game_area.1.into()
    {
        return GameCell::Edge;
    }
    if game_state.player.head_position.column_number == column_number
        && game_state.player.head_position.row_number == row_number
    {
        return GameCell::Head;
    }
    if game_state.powerup.column_number == column_number
        && game_state.powerup.row_number == row_number
    {
        return GameCell::Powerup;
    }
    if game_state.tail.check(column_number, row_number) {
        return GameCell::Tail;
    }

    return GameCell::Empty;
}

fn generate_game_visual(game_state: &GameState) -> String {
    let mut rows: Vec<String> = Vec::new();
    let horizontal_border = "-".repeat(game_state.game_area.0.into());
    let normal_zone_columns = game_state.game_area.0 - 2;
    let normal_zone_rows = game_state.game_area.1 - 2;
    rows.push(horizontal_border.clone());

    for row_number in 1..=normal_zone_rows {
        let mut row = String::new();
        row.push('|');
        for column_number in 1..=normal_zone_columns {
            let cell: GameCell =
                determine_game_cell(&game_state, column_number.into(), row_number.into());
            match cell {
                GameCell::Head => row.push('S'),
                GameCell::Tail => row.push('O'),
                GameCell::Powerup => row.push('*'),
                GameCell::Empty => row.push(' '),
                GameCell::Edge => (),
            }
        }
        row.push('|');
        rows.push(row);
    }

    rows.push(horizontal_border.clone());
    return rows.join("\n");
}

impl GameState {
    pub fn new() -> Self {
        let player = Player::new(40, 15);
        let tail = Tail::new(39, 15);
        let game_area = GameArea(80, 30);
        let powerup = Position {
            column_number: 10,
            row_number: 10,
        };
        return Self {
            player,
            tail,
            game_area,
            powerup,
        };
    }
    pub fn run(&mut self) {
        while let Some(_) = self.tick() {
            // handle input
            self.render();
            self.handle_input();
        }
    }
    fn handle_input(&mut self) {
        let input = read_input().unwrap_or("*".into());
        let direction: Option<Direction> = match input.as_ref() {
            "h" => Some(Direction::Left),
            "j" => Some(Direction::Down),
            "k" => Some(Direction::Up),
            "l" => Some(Direction::Right),
            _ => None,
        };
        if let Some(dir) = direction {
            self.player.change_heading(dir);
        }
    }

    fn tick(&mut self) -> Option<()> {
        let next_position = self.player.calculate_new_position();

        let next_game_cell: GameCell =
            determine_game_cell(&self, next_position.column_number, next_position.row_number);

        match next_game_cell {
            GameCell::Empty => {
                let previous_position = self.player.move_player(next_position);
                self.tail.positions.push_front(previous_position);
                let _ = self.tail.positions.pop_back();
                Some(())
            }
            GameCell::Powerup => {
                let previous_position = self.player.move_player(next_position);
                self.tail.positions.push_front(previous_position);
                // TODO: change Powerup position
                Some(())
            }
            GameCell::Tail => None,
            GameCell::Edge => None,
            GameCell::Head => {
                panic!("impossible behaviour");
            }
        }
    }

    fn render(&self) {
        let visuals = generate_game_visual(self);
        println!("{}", visuals);
    }
}

#[cfg(test)]
mod tests {
    use super::GameState;
    use crate::{
        game::{generate_game_visual, GameArea},
        player::{Player, Position, Tail},
    };

    #[test]
    fn generate_game_visual_produces_expected_string() {
        let game_state = GameState {
            player: Player::new(3, 1),
            tail: Tail::new(2, 1),
            powerup: Position {
                column_number: 1,
                row_number: 1,
            },
            game_area: GameArea(5, 3),
        };
        // 6 x 6
        let expected_output = r#"
-----
|*OS|
-----
"#
        .trim();
        assert_eq!(&generate_game_visual(&game_state), expected_output);
    }
}
