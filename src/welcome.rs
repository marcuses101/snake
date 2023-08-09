use crate::constants::*;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

const WELCOME_MESSAGE: &str = "HAPPY BIRTHDAY LAURENCE!!!";

fn get_inner_row(text: Option<&str>, width: u8) -> String {
    let mut output = String::new();
    output.push(VERTICAL_WALL);
    match text {
        None => {
            let number_of_inner_characters = width - 2;
            let inner = " ".repeat(number_of_inner_characters.into());
            output.push_str(&inner);
        }
        Some(text) => {
            let text_length: u8 = text.len().try_into().unwrap();
            let total_space_characters: u8 = width - text_length - 2;
            if total_space_characters % 2 == 0 {
                let spaces = " ".repeat((total_space_characters / 2).into());
                output.push_str(&spaces);
                output.push_str(text);
                output.push_str(&spaces);
            } else {
                let spaces = " ".repeat((total_space_characters / 2).into());
                output.push_str(&spaces);
                output.push_str(text);
                output.push(' ');
                output.push_str(&spaces);
            }
        }
    }
    output.push(VERTICAL_WALL);
    output
}

pub fn display_welcome_screen(width: u8, height: u8) -> Option<()> {
    let top_border = format!(
        "{}{}{}",
        TOP_LEFT_CORNER,
        HORIZONTAL_WALL.to_string().repeat((width - 2).into()),
        TOP_RIGHT_CORNER
    );
    let bottom_border = format!(
        "{}{}{}",
        BOTTOM_LEFT_CORNER,
        HORIZONTAL_WALL.to_string().repeat((width - 2).into()),
        BOTTOM_RIGHT_CORNER
    );

    let word_rows: Vec<String> = WELCOME_MESSAGE
        .split(' ')
        .flat_map(|word| vec![get_inner_row(Some(word), width), get_inner_row(None, width)])
        .collect();
    let word_rows_length: u8 = word_rows.len().try_into().unwrap();
    let number_of_empty_rows: u8 = height - 2 - word_rows_length;

    let empty_row = get_inner_row(None, width);
    let half_of_empty_rows = vec![empty_row; (number_of_empty_rows / 2).into()];

    let mut rows = vec![top_border];
    rows.extend(half_of_empty_rows.clone());
    rows.extend(word_rows);
    rows.extend(half_of_empty_rows.clone());
    rows.push(bottom_border);

    if number_of_empty_rows % 2 > 0 {
        rows.insert(1, get_inner_row(None, width));
    }

    let welcome = rows.join("\r\n");

    print!("{}{}{}\n\r", clear::All, cursor::Goto(1, 1), welcome);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') | Key::Ctrl('c') => return None,
            Key::Char(_) => {
                break;
            }
            _ => (),
        }
    }
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
    Some(())
}

#[cfg(test)]
mod tests {
    use super::get_inner_row;

    #[test]
    fn get_row_returns_string_of_expected_length_without_input() {
        let row: String = get_inner_row(None, 10);
        assert_eq!(row.len(), 10);
    }

    #[test]
    fn get_row_returns_string_of_expected_length_with_input() {
        let row: String = get_inner_row(Some("TEST"), 10);
        assert_eq!(row.len(), 10);
    }

    #[test]
    fn get_row_returns_expected_string_with_input() {
        let row: String = get_inner_row(Some("WELCOME"), 13);
        assert_eq!(&row, "+  WELCOME  +");
    }

    #[test]
    fn get_row_returns_expected_string_with_no_input() {
        let row: String = get_inner_row(None, 13);
        assert_eq!(&row, "+           +");
    }
}
