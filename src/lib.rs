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

pub mod erasable_cluster;
mod errors;
mod helpers;
mod named_constants;
mod parsing_calculator;
mod rational_number;
mod sign;

pub type UnsignedValueDepth = u64;
pub type SignedValueDepth = i64;

pub const PI: char = 'π';

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
