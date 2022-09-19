use calculator::{erasable_cluster::ErasableCluster, exit_if_error, print};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, MouseEventKind},
    execute,
    style::Print,
    terminal::enable_raw_mode,
};
use std::{io::stdout, process};

// const BACKSPACE: char = 8u8 as char;

fn main() {
    exit_if_error(enable_raw_mode());

    print("The calculator you never knew you needed (until you started calculus)\n");
    exit_if_error(execute!(stdout(), cursor::SavePosition));

    let cluster = ErasableCluster::new();

    loop {
        match exit_if_error(read()) {
            // Event::FocusGained => println!("FocusGained"),
            // Event::FocusLost => println!("FocusLost"),
            // Event::Resize(width, height) => println!("New size {}x{}", width, height),
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => {}
                _ => {}
            },
            #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("{:?}", data),
            _ => (),
        }

        exit_if_error(execute!(
            stdout(),
            cursor::RestorePosition,
            // Print(&expression),
        ));
        // match io::stdout() {
        //     Ok(c) => {
        //         match c {
        //             '0'..='9' => expression.push(c),
        //             'q' => {
        //                 println!("Exiting");
        //                 process::exit(0)
        //             }
        //             _ => panic!("Unknown character entered"),
        //         }

        //         println!("{expression}")
        //     }
        //     Err(e) => panic!("{e}"),
        // }
    }
}
