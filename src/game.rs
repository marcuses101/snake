use crate::{
    player::{Direction, Player, Position, Tail},
    utils::read_input,
};
use array2d::Array2D;
use color_eyre::eyre::{eyre, ErrReport, Result};
use rand::Rng;
use termion::cursor;

pub struct GameBoard(pub Array2D<GameCell>);

pub struct GameArea {
    pub width: u8,
    pub height: u8,
}

pub struct GameState {
    player: Player,
    tail: Tail,
    pub game_area: GameArea,
    powerup: Powerup,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

struct Powerup(Position);

impl Powerup {
    pub fn new(column_number: isize, row_number: isize) -> Self {
        Self(Position::new(column_number, row_number))
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum GameCell {
    Head,
    Tail,
    Powerup,
    Empty,
    Edge,
}

pub fn determine_game_cell(
    game_state: &GameState,
    column_number: isize,
    row_number: isize,
) -> GameCell {
    if column_number == 0
        || row_number == 0
        || column_number + 1 >= game_state.game_area.width.into()
        || row_number + 1 >= game_state.game_area.height.into()
    {
        return GameCell::Edge;
    }
    if game_state.player.head_position.column_number == column_number
        && game_state.player.head_position.row_number == row_number
    {
        return GameCell::Head;
    }
    if game_state.powerup.0.column_number == column_number
        && game_state.powerup.0.row_number == row_number
    {
        return GameCell::Powerup;
    }
    if game_state.tail.check(column_number, row_number) {
        return GameCell::Tail;
    }

    GameCell::Empty
}

impl GameState {
    pub fn new() -> Self {
        let player = Player::new(40, 15);
        let tail = Tail::new(39, 15);
        let game_area = GameArea {
            width: 80,
            height: 30,
        };
        let powerup = Powerup::new(10, 10);

        Self {
            player,
            tail,
            game_area,
            powerup,
        }
    }

    fn randomize_powerup_position(&mut self) {
        let mut rng = rand::thread_rng();
        let mut powerup_column: isize = rng.gen_range(1..self.game_area.width).into();
        let mut powerup_row: isize = rng.gen_range(1..self.game_area.height).into();
        while determine_game_cell(self, powerup_column, powerup_row) != GameCell::Empty {
            powerup_column = rng.gen_range(1..self.game_area.width).into();
            powerup_row = rng.gen_range(1..self.game_area.height).into();
        }
        self.powerup = Powerup::new(powerup_column, powerup_row);
    }

    pub fn run(&mut self) {
        while self.tick().is_some() {
            // handle input
            self.render();
            self.handle_input();
        }
    }

    fn handle_input(&mut self) {
        let input = read_input().unwrap_or("".into());
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
            determine_game_cell(self, next_position.column_number, next_position.row_number);

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
                self.randomize_powerup_position();
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
        let game_board_result = GameBoard::try_from(self);
        if let Ok(game_board) = game_board_result {
            print!(
                "{}{}{}",
                termion::clear::All,
                cursor::Goto(1, 1),
                game_board
            );
        }
    }
}

impl TryFrom<&GameState> for GameBoard {
    type Error = ErrReport;
    fn try_from(value: &GameState) -> Result<Self> {
        let rows: Vec<Vec<GameCell>> = (0..value.game_area.height)
            .map(|row_index| {
                let row: Vec<GameCell> = (0..value.game_area.width)
                    .map(|column_index| {
                        determine_game_cell(value, column_index.into(), row_index.into())
                    })
                    .collect();
                row
            })
            .collect();
        let two_dimensional_array =
            Array2D::from_rows(&rows).map_err(|_| eyre!("unable to construct"))?;
        Ok(GameBoard(two_dimensional_array))
    }
}

#[cfg(test)]
mod tests {}
