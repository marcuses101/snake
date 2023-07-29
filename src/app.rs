use crate::{game::GameState, welcome::WELCOME_SCREEN};
use std::io::{stdin, stdout, Write};
use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

enum Screen {
    Welcome,
    GamePlay,
    Score,
}

pub struct App {
    state: Screen,
    game_state: GameState,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            state: Screen::Welcome,
            game_state: GameState::new(),
        }
    }
    fn change_screen(&mut self) {
        self.state = match self.state {
            Screen::Welcome => Screen::GamePlay,
            Screen::GamePlay => Screen::Score,
            Screen::Score => Screen::Welcome,
        };
    }
    pub fn run(&mut self) {
        loop {
            match self.state {
                Screen::Welcome => {
                    print!(
                        "{}{}{}",
                        clear::All,
                        cursor::Goto(1, 1),
                        WELCOME_SCREEN.trim_start(),
                    );
                    let stdin = stdin();
                    let mut stdout = stdout().into_raw_mode().unwrap();
                    for c in stdin.keys() {
                        match c.unwrap() {
                            Key::Char('q') | Key::Ctrl('c') => {
                                drop(stdout);
                                process::exit(0);
                            }
                            Key::Char(_) => {
                                break;
                            }
                            _ => (),
                        }
                    }
                    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                    stdout.flush().unwrap();
                    self.change_screen();
                }
                Screen::GamePlay => {
                    self.game_state.run();
                    self.change_screen();
                    self.game_state = GameState::new()
                }
                Screen::Score => {
                    self.change_screen();
                }
            };
        }
    }
}
