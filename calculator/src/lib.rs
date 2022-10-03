pub mod calculation;
mod display;
pub mod input_parsing;
mod shared;

#[macro_use]
extern crate num_derive;

use std::{fmt::Display, io::stdout};

use crossterm::{execute, style::Print};

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn display_help_text() -> io::Result<()> {
    let file = File::open("help_text.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println(&(line?))?;
    }

    Ok(())
}

pub struct OnScreenCursorCoordinates {
    pub from_left: u16,
    pub from_top: u16,
}

pub fn print<T: Display>(msg: T) -> Result<(), io::Error> {
    execute!(stdout(), Print(msg))
}

pub fn println<T: Display>(msg: T) -> Result<(), io::Error> {
    print(msg)?;
    print("\n")?;

    Ok(())
}
