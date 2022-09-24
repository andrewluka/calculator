#[macro_use]
extern crate num_derive;

use std::{
    fmt::{Debug, Display},
    io::stdout,
    process,
};

use crossterm::{
    execute,
    style::Print,
    terminal::{disable_raw_mode, is_raw_mode_enabled},
};

mod calculation;
mod display;
pub mod input_parsing;
mod shared;

pub fn exit_if_error<T, E: Debug>(r: Result<T, E>) -> T {
    match r {
        Ok(value) => value,
        Err(e) => {
            if is_raw_mode_enabled().expect("Internal error") {
                disable_raw_mode().expect("Internal error");
            }

            eprintln!("Internal error: {:#?}", e);
            process::exit(1);
        }
    }
}

pub fn print<T: Display>(msg: T) {
    exit_if_error(execute!(stdout(), Print(msg)));
}

pub fn println(msg: &str) {
    print(msg);
    print("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_if_no_error() {
        let r: Result<u8, &str> = Ok(89);
        assert_eq!(exit_if_error(r), 89);
    }
}
