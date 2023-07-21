use crate::{game::GameState, welcome::WELCOME_SCREEN};

enum AppState {
    WelcomeScreen,
    GamePlayScreen,
    ScoreScreen,
}

pub struct App {
    state: AppState,
    game_state: GameState,
}

impl App {
    pub fn new() -> Self {
        return Self {
            state: AppState::WelcomeScreen,
            game_state: GameState::new(),
        };
    }
    fn change_screen(&mut self) {
        self.state = match self.state {
            AppState::WelcomeScreen => AppState::GamePlayScreen,
            AppState::GamePlayScreen => AppState::ScoreScreen,
            AppState::ScoreScreen => AppState::WelcomeScreen,
        };
    }
    pub fn run(&mut self) {
        loop {
            match self.state {
                AppState::WelcomeScreen => {
                    println!("{}", WELCOME_SCREEN);
                }
                AppState::GamePlayScreen => {
                    self.game_state.run();
                    self.change_screen();
                }
                AppState::ScoreScreen => {
                    self.change_screen();
                }
            };
        }
    }
}
