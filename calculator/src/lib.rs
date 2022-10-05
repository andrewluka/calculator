pub mod calculation;
mod display;
pub mod input_parsing;
mod shared;

#[macro_use]
extern crate num_derive;

use std::{fmt::Display, io::stdout};

use crossterm::{execute, style::Print};

use std::fs::File;
use std::io::{self, prelude::*, stderr, BufReader};

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

// to be used when terminal raw mode is enabled
pub fn print<T: Display>(msg: T) -> Result<(), io::Error> {
    execute!(stdout(), Print(msg))
}

// to be used when terminal raw mode is enabled
pub fn println<T: Display>(msg: T) -> Result<(), io::Error> {
    print(msg)?;
    // carrier return '\r' so output is proper on linux
    // (in raw mode, the cursor is not directly brought back to
    // the beginning of the next line)
    print("\n\r")?;

    Ok(())
}

// to be used when terminal raw mode is enabled
pub fn eprint<T: Display>(msg: T) -> Result<(), io::Error> {
    // again, carrier return '\r' so it works on linux
    write!(stderr(), "\r\n\n{msg}\n\n\r")?;

    Ok(())
}
