use crate::{game::GameState, utils::read_input, welcome::WELCOME_SCREEN};

enum Screen {
    Welcome,
    GamePlay,
    Score,
}

pub struct App {
    state: Screen,
    game_state: GameState,
}

impl App {
    pub fn new() -> Self {
        return Self {
            state: Screen::Welcome,
            game_state: GameState::new(),
        };
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
                    println!("{}", WELCOME_SCREEN);
                    read_input().ok();
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
