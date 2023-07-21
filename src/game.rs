use crate::{player::Player, welcome::WELCOME_SCREEN};

enum GameState {
    WelcomeScreen,
    GamePlayScreen,
    ScoreScreen,
}

pub struct App {
    state: GameState,
}

impl App {
    pub fn new() -> Self {
        return Self {
            state: GameState::WelcomeScreen,
        };
    }
    fn change_screen(&mut self) {
        self.state = match self.state {
            GameState::WelcomeScreen => GameState::GamePlayScreen,
            GameState::GamePlayScreen => GameState::ScoreScreen,
            GameState::ScoreScreen => GameState::WelcomeScreen,
        };
    }
    pub fn run(&self) {
        match self.state {
            GameState::WelcomeScreen => {
                println!("{}", WELCOME_SCREEN);
            }
            GameState::GamePlayScreen => {
                todo!()
            }
            GameState::ScoreScreen => {
                todo!()
            }
        };
    }
}

struct GameArea(u8, u8);

struct GameManager {
    player: Player,
    game_area: GameArea,
}

impl GameManager {
    fn new() -> Self {
        let player = Player::new();
        let game_area = GameArea(50, 50);
        return Self { player, game_area };
    }
    fn tick() -> () {
        todo!();
    }
}
