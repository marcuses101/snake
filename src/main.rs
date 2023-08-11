extern crate termion;

use color_eyre::eyre::Result;
use snake::app::App;
use std::fs;

const MIN_TERMINAL_COLUMNS: usize = 80;
const MIN_TERMINAL_ROWS: usize = 30;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const HIGH_SCORE_FILE_PATH: &str = "high_score.txt";

struct Config {
    high_score: usize,
    width: usize,
    height: usize,
}

impl Config {
    fn build() -> Result<Self> {
        let high_score: usize = fs::read_to_string(HIGH_SCORE_FILE_PATH)?
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
    if usize::from(terminal_columns) < width || usize::from(terminal_rows) < height {
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
