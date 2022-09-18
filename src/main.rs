use calculator;
use crossterm::terminal::enable_raw_mode;
use std::{io, process};

// const BACKSPACE: char = 8u8 as char;

fn main() {
    println!("The calculator you never knew you needed (until you started calculus)");
    let mut expression = String::new();
    match enable_raw_mode() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Internal error: {}", e);
            process::exit(1);
        }
    }

    // loop {
    //     match io::stdout() {
    //         Ok(c) => {
    //             match c {
    //                 '0'..='9' => expression.push(c),
    //                 'q' => {
    //                     println!("Exiting");
    //                     process::exit(0)
    //                 }
    //                 _ => panic!("Unknown character entered"),
    //             }

    //             println!("{expression}")
    //         }
    //         Err(e) => panic!("{e}"),
    //     }
    // }
}
