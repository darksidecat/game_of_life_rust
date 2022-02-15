use crate::game_of_life::{Cell, GameField};
use crossterm::style::{Color, SetForegroundColor};
use crossterm::{cursor, ExecutableCommand};
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

pub fn prepare_console() {
    let mut stdout = stdout();
    match stdout.execute(cursor::Hide) {
        Ok(_) => (),
        Err(error) => {
            println!("Failed to hide cursor. {}", error)
        }
    };
    match stdout.execute(SetForegroundColor(Color::Green)) {
        Ok(_) => (),
        Err(error) => {
            println!("Failed to set background color. {}", error)
        }
    }
}

pub fn render_field(game: &mut GameField, sleep_time: u64) {
    game.update_field();
    draw_field(game.current_field());
    sleep(Duration::from_millis(sleep_time));
    clearscreen::clear().expect("Failed to clear display");
}

pub fn draw_field(field: &Vec<Vec<Cell>>) {
    let mut buffer = String::new();
    for row in field {
        for cell in row {
            buffer.push_str(if *cell == Cell::Alive { "#" } else { " " });
        }
        buffer.push_str("\n");
    }
    println!("{}", &buffer);
}
