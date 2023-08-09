extern crate termion;

use color_eyre::eyre::Result;
use snake::app::App;
use std::fs;

const MIN_TERMINAL_COLUMNS: u16 = 80;
const MIN_TERMINAL_ROWS: u16 = 30;

const WIDTH: u8 = 80;
const HEIGHT: u8 = 25;
const HIGH_SCORE_FILE_PATH: &str = "high_score.txt";

struct Config {
    high_score: u16,
    width: u8,
    height: u8,
}

impl Config {
    fn build() -> Result<Self> {
        let high_score: u16 = fs::read_to_string(HIGH_SCORE_FILE_PATH)?
            .trim()
            .parse()
            .unwrap_or(0);
        Ok(Config {
            high_score,
            width: WIDTH,
            height: HEIGHT,
        })
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let Config {
        high_score,
        width,
        height,
    } = Config::build()?;
    let (terminal_columns, terminal_rows) =
        termion::terminal_size().expect("unable to determine terminal size");
    if terminal_columns < width.into() || terminal_rows < height.into() {
        eprintln!(
            "\
            This app required a terminal of {} x {}\ncurrent dimensions: {} x {}",
            MIN_TERMINAL_COLUMNS, MIN_TERMINAL_ROWS, terminal_columns, terminal_rows
        );
        panic!();
    }

    let mut app = App::new(high_score, width, height);
    app.run();
    Ok(())
}
