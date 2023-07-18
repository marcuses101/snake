extern crate termion;

use snake::player::{Direction, Player};
use std::io;

fn read_input() -> io::Result<String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    println!("You typed: {}", input);

    Ok(input.into())
}

fn main() {
    let mut player: Player = Player::new();
    let terminal_size = termion::terminal_size().unwrap();
    dbg!(terminal_size);
    println!("{}", &player.position);
    player.move_player();
    println!("{}", &player.position);
    loop {
        let user_input = read_input().unwrap();
        match Direction::try_from(user_input) {
            Ok(dir) => {
                player.change_direction(dir);
                player.move_player();
            }
            Err(error) => {
                eprint!("{}", error);
            }
        };
        println!("{}", &player.position);
    }
}
