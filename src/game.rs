use crate::player::{Direction, Player, Position, Tail};
use array2d::Array2D;
use color_eyre::eyre::{eyre, ErrReport, Result};
use rand::Rng;
use std::io::{stdout, StdoutLock, Write};
use std::process;
use std::{thread, time::Duration};
use termion::raw::RawTerminal;
use termion::{async_stdin, cursor, event::Key, input::TermRead, raw::IntoRawMode};

pub struct GameBoard(pub Array2D<GameCell>);

#[derive(Clone, Copy)]
pub struct GameArea {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone)]
pub struct GameState {
    player: Player,
    tail: Tail,
    pub game_area: GameArea,
    powerup: Powerup,
    score: usize,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(60, 25)
    }
}

#[derive(Clone, Copy)]
struct Powerup(Position);

impl Powerup {
    pub fn new(column_number: usize, row_number: usize) -> Self {
        Self(Position::new(column_number, row_number))
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Wall {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(PartialEq, Clone, Copy)]
pub enum GameCell {
    Head(Direction),
    Tail,
    Powerup,
    Empty,
    Edge(Wall),
}

pub fn determine_game_cell(
    game_state: &GameState,
    column_number: usize,
    row_number: usize,
) -> GameCell {
    let right_edge = game_state.game_area.width - 1;
    let bottom_edge = game_state.game_area.height - 1;

    match (column_number, row_number) {
        (0, 0) => GameCell::Edge(Wall::TopLeft),
        (c, 0) if c == right_edge => GameCell::Edge(Wall::TopRight),
        (0, r) if r == bottom_edge => GameCell::Edge(Wall::BottomLeft),
        (c, r) if c == right_edge && r == bottom_edge => GameCell::Edge(Wall::BottomRight),
        (0, _) => GameCell::Edge(Wall::Vertical),
        (c, _) if c == right_edge => GameCell::Edge(Wall::Vertical),
        (_, 0) => GameCell::Edge(Wall::Horizontal),
        (_, r) if r == bottom_edge => GameCell::Edge(Wall::Horizontal),
        _ if game_state.player.head_position.column_number == column_number
            && game_state.player.head_position.row_number == row_number =>
        {
            GameCell::Head(game_state.player.heading)
        }
        _ if game_state.powerup.0.column_number == column_number
            && game_state.powerup.0.row_number == row_number =>
        {
            GameCell::Powerup
        }
        _ if game_state.tail.check(column_number, row_number) => GameCell::Tail,
        _ => GameCell::Empty,
    }
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let player_x = width / 2;
        let player_y = height / 2;
        let tail_x = player_x - 1;
        let tail_y = player_y;
        let player = Player::new(player_x, player_y);
        let tail = Tail::new(tail_x, tail_y);
        let game_area = GameArea { width, height };
        let powerup = Powerup::new(10, 10);

        Self {
            player,
            tail,
            game_area,
            powerup,
            score: 0,
        }
    }

    fn randomize_powerup_position(&mut self) {
        let mut rng = rand::thread_rng();
        let mut powerup_column: usize = rng.gen_range(1..self.game_area.width);
        let mut powerup_row: usize = rng.gen_range(1..self.game_area.height);
        while determine_game_cell(self, powerup_column, powerup_row) != GameCell::Empty {
            powerup_column = rng.gen_range(1..self.game_area.width);
            powerup_row = rng.gen_range(1..self.game_area.height);
        }
        self.powerup = Powerup::new(powerup_column, powerup_row);
    }

    pub fn run(&mut self) -> Result<usize> {
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode()?;
        let mut input = async_stdin().keys();
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            cursor::Goto(1, 1),
            cursor::Hide
        )?;
        stdout.flush()?;
        loop {
            let last_key = input.by_ref().last().unwrap_or(Ok(Key::Null))?;
            if let Key::Char('q') = last_key {
                drop(stdout);
                process::exit(0);
            }
            self.handle_input(last_key);

            if self.tick().is_none() {
                break;
            }
            self.render(stdout.by_ref())?;
            thread::sleep(Duration::from_millis(75));
        }
        write!(stdout, "{}", cursor::Show)?;
        stdout.flush()?;
        Ok(self.score)
    }

    fn render(&self, stdout: &mut RawTerminal<StdoutLock<'_>>) -> Result<()> {
        let game_board = GameBoard::try_from(self)?;
        write!(stdout, "{}{}", cursor::Goto(1, 1), game_board,)?;
        stdout.flush()?;
        Ok(())
    }

    fn handle_input(&mut self, input: Key) {
        let direction = match input {
            Key::Char('h') | Key::Left => Some(Direction::Left),
            Key::Char('j') | Key::Down => Some(Direction::Down),
            Key::Char('k') | Key::Up => Some(Direction::Up),
            Key::Char('l') | Key::Right => Some(Direction::Right),
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
                self.score += 1;
                let previous_position = self.player.move_player(next_position);
                self.tail.positions.push_front(previous_position);
                self.randomize_powerup_position();
                Some(())
            }
            GameCell::Tail => None,
            GameCell::Edge(_) => None,
            GameCell::Head(_) => {
                panic!("impossible behaviour");
            }
        }
    }
}

impl TryFrom<&GameState> for GameBoard {
    type Error = ErrReport;
    fn try_from(value: &GameState) -> Result<Self> {
        let rows: Vec<Vec<GameCell>> = (0..value.game_area.height)
            .map(|row_index| {
                let row: Vec<GameCell> = (0..value.game_area.width)
                    .map(|column_index| determine_game_cell(value, column_index, row_index))
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
