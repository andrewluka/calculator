mod calculation;
mod display;
pub mod input_parsing;
mod shared;

#[macro_use]
extern crate num_derive;

use std::{
    fmt::{Debug, Display},
    io::stdout,
    process,
};

use crossterm::{
    cursor, execute, queue,
    style::Print,
    terminal::{disable_raw_mode, is_raw_mode_enabled},
};

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn display_help_text() -> io::Result<()> {
    let file = File::open("help_text.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println(&(line?));
    }

    Ok(())
}

pub struct OnScreenCursorCoordinates {
    pub from_left: u16,
    pub from_top: u16,
}

// pub fn exit_if_error<T, E: Debug>(r: Result<T, E>) -> T {
//     match r {
//         Ok(value) => value,
//         Err(e) => {
//             if is_raw_mode_enabled().expect("Internal error") {
//                 disable_raw_mode().expect("Internal error");
//             }

//             eprintln!("Internal error: {:#?}", e);
//             process::exit(1);
//         }
//     }
// }

pub fn print<T: Display>(msg: T) -> Result<(), io::Error> {
    execute!(stdout(), Print(msg))
}

pub fn println(msg: &str) -> Result<(), io::Error> {
    print(msg)?;
    print("\n")?;

    Ok(())
}
