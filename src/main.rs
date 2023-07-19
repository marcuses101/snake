extern crate termion;

use snake::game::App;
use std::io;

fn read_input() -> io::Result<String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    println!("You typed: {}", input);

    Ok(input.into())
}

fn main() {
    let app = App::new();
}
