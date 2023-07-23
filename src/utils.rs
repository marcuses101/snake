use std::io;

pub fn read_input() -> io::Result<String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    Ok(input.into())
}
