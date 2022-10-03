use calculator::{
    calculation::calculator::Calculator, display_help_text,
    input_parsing::erasable_cluster::ErasableCluster, print, println,
};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, queue,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{stdout, Write},
    process,
};

// const BACKSPACE: char = 8u8 as char;

fn rerender(output: &str, root_position: (u16, u16)) -> Result<(), std::io::Error> {
    let mut stdout = stdout();

    disable_raw_mode()?;

    queue!(stdout, cursor::MoveTo(root_position.0, root_position.1))?;
    queue!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown))?;

    stdout.flush()?;

    print(&output)?;
    enable_raw_mode()?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut cluster = ErasableCluster::new();
    let mut output;

    println("The calculator you never knew you needed (until you started calculus).")?;
    println("For help, press h. To quit, press q.")?;

    let mut root_position = cursor::position()?;

    execute!(stdout(), cursor::SavePosition)?;

    enable_raw_mode()?;

    loop {
        // print(cluster.to_string());

        let do_trigger_a_rerender = match read()? {
            // Event::FocusGained => println!("FocusGained"),
            // Event::FocusLost => println!("FocusLost"),
            // Event::Resize(width, height) => println!("New size {}x{}", width, height),
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => match c {
                    'q' => {
                        println("\nSee ya later!")?;
                        disable_raw_mode()?;
                        process::exit(0)
                    }
                    'h' => {
                        if let Err(_) = display_help_text() {
                            println("unable to display help text")?;
                        }
                        root_position = cursor::position()?;
                        false
                    }
                    _ => {
                        match cluster.add_at_cursor_position(c) {
                            Ok(e) => execute!(
                                stdout(),
                                cursor::MoveRight(e.length_in_chars() as u16),
                                cursor::SavePosition,
                            )?,
                            Err(_) => {
                                println(&format!("\nunknown character: {}", c))?;
                                root_position = cursor::position()?;
                            }
                        }
                        true
                    }
                },
                KeyCode::Backspace => {
                    match cluster.remove_at_cursor_position() {
                        Ok(e) => execute!(
                            stdout(),
                            cursor::MoveLeft(e.length_in_chars() as u16),
                            cursor::SavePosition,
                        )?,
                        Err(_) => (),
                    }

                    true
                }
                KeyCode::Left => {
                    match cluster.move_cursor_to_prev_erasable() {
                        Some(e) => execute!(
                            stdout(),
                            cursor::MoveLeft(e.length_in_chars() as u16),
                            cursor::SavePosition,
                        )?,
                        None => (),
                    };
                    false
                }
                KeyCode::Right => {
                    match cluster.move_cursor_to_next_erasable() {
                        Some(e) => {
                            execute!(
                                stdout(),
                                cursor::MoveRight(e.length_in_chars() as u16),
                                cursor::SavePosition,
                            )?;
                        }
                        None => (),
                    };
                    false
                }
                KeyCode::Enter => {
                    let mut calc = Calculator::from(&cluster);
                    cluster = ErasableCluster::new();

                    print!("\n\n");
                    print(calc.next_inexact_output_mode())?;
                    print!("\n\n");

                    root_position = cursor::position()?;

                    true
                }
                _ => {
                    // println(&format!("{:#?}", event));
                    false
                }
            },
            #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("{:?}", data),
            _ => false,
        };

        if do_trigger_a_rerender {
            output = cluster.to_string();
            rerender(&output, root_position)?;
            execute!(stdout(), cursor::RestorePosition)?;
        }
    }
}
