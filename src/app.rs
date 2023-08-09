use crate::game::GameState;
use crate::welcome::display_welcome_screen;
use std::fs;
use std::io::{stdin, stdout, Write};
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
    score: Option<u16>,
    high_score: u16,
    width: u8,
    height: u8,
}

impl Default for App {
    fn default() -> Self {
        Self::new(0, 60, 25)
    }
}

impl App {
    pub fn new(high_score: u16, width: u8, height: u8) -> Self {
        Self {
            state: Screen::Welcome,
            game_state: GameState::new(width, height),
            score: None,
            high_score,
            width,
            height,
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
        'game_loop: loop {
            match self.state {
                Screen::Welcome => {
                    print!("{}{}", clear::All, cursor::Goto(1, 1));
                    if display_welcome_screen(self.width, self.height).is_none() {
                        break 'game_loop;
                    }
                    self.change_screen();
                }
                Screen::GamePlay => {
                    let score = self.game_state.run().unwrap();
                    self.score = Some(score);
                    self.change_screen();
                    self.game_state = GameState::new(self.width, self.height)
                }
                Screen::Score => {
                    let stdin = stdin();
                    let mut stdout = stdout().into_raw_mode().unwrap();
                    if let Some(score) = self.score {
                        if score > self.high_score {
                            let content = score.to_string();
                            fs::write("high_score.txt", content).unwrap();
                            write!(
                                stdout,
                                "{}{}YOU GOT THE NEW HIGH SCORE: {}!!! \n\r\n\rTHE PREVIOUS HIGH SCRORE WAS {}\n\r\n\rPress the SPACEBAR to contiue",
                                clear::All,
                                cursor::Goto(1, 1),
                                score,
                                self.high_score
                                )
                                .unwrap();
                            self.high_score = score;
                        } else {
                            write!(
                                stdout,
                                "{}{}YOU SCORED: {}!!! \n\r\n\rHIGH SCORE: {}\n\r\n\rPress the SPACEBAR to contiue",
                                clear::All,
                                cursor::Goto(1, 1),
                                score,
                                self.high_score
                            )
                            .unwrap();
                        }
                    }
                    stdout.flush().unwrap();
                    for c in stdin.keys() {
                        match c.unwrap() {
                            Key::Char('q') | Key::Ctrl('c') => {
                                break 'game_loop;
                            }
                            Key::Char(' ') => {
                                break;
                            }
                            _ => (),
                        }
                    }
                    self.change_screen();
                }
            };
        }
    }
}
