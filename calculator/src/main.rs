use calculator::{erasable_cluster::ErasableCluster, exit_if_error, print, println};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, MouseEventKind},
    execute,
    terminal::enable_raw_mode,
};
use std::io::stdout;

// const BACKSPACE: char = 8u8 as char;

fn main() {
    exit_if_error(enable_raw_mode());

    println("The calculator you never knew you needed (until you started calculus).");
    println("For help, press h. To quit, press q.");

    exit_if_error(execute!(stdout(), cursor::SavePosition));

    let mut cluster = ErasableCluster::new();

    loop {
        print(cluster.to_string());

        match exit_if_error(read()) {
            // Event::FocusGained => println!("FocusGained"),
            // Event::FocusLost => println!("FocusLost"),
            // Event::Resize(width, height) => println!("New size {}x{}", width, height),
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => {
                    cluster.add_at_cursor_position(c);
                }
                _ => {
                    // println(&format!("{:#?}", event));
                }
            },
            #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("{:?}", data),
            _ => (),
        }

        exit_if_error(execute!(stdout(), cursor::RestorePosition));
    }
}
