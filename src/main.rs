extern crate termion;

use snake::app::App;
use std::io;

const MIN_TERMINAL_COLUMNS: u16 = 80;
const MIN_TERMINAL_ROWS: u16 = 30;

fn main() {
    let (terminal_columns, terminal_rows) =
        termion::terminal_size().expect("unable to determine terminal size");
    if terminal_columns < MIN_TERMINAL_COLUMNS || terminal_rows < MIN_TERMINAL_ROWS {
        eprintln!(
            "\
            This app required a terminal of {} x {}\ncurrent dimensions: {} x {}",
            MIN_TERMINAL_COLUMNS, MIN_TERMINAL_ROWS, terminal_columns, terminal_rows
        );
        panic!();
    }
    let mut app = App::new();
    app.run();
}
