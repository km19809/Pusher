//! Simple CLI clone of sokoban

#[cfg(feature = "tui")]
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};
use std::error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
mod stage;
mod vector2;

use stage::{Direction, Stage};
pub use vector2::Vector2;
/// Struct for contain parsed arguments.
pub struct Arguments {
    /// File name of stage data
    pub filename: String,
}

/// Handles input and update stage.
/// # Returns
/// `true` if user wants to continue, else `false`
/// # Errors
/// It returns `Err(&'static str)` when:
/// * Error occured while moving player
/// * Input is invalid or empty
fn update(s: &mut Stage) -> Result<bool, &'static str> {
    #[cfg(not(feature = "tui"))]
    {
        let mut inputs = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut inputs)
            .expect("Failed to read input.");
        let input = inputs.trim().chars().next();
        match input {
            Some(cmd) => match cmd {
                'W' | 'w' | 'K' | 'k' => s.move_player(Direction::Up).map(|()| true),
                'A' | 'a' | 'H' | 'h' => s.move_player(Direction::Left).map(|()| true),
                'S' | 's' | 'J' | 'j' => s.move_player(Direction::Down).map(|()| true),
                'D' | 'd' | 'L' | 'l' => s.move_player(Direction::Right).map(|()| true),
                'Q' | 'q' => Ok(false),
                _ => Err("Invalid input."),
            },
            None => Err("Empty input."),
        }
    }
    #[cfg(feature = "tui")]
    {
        let input = read().map_err(|_err| "Invalid input")?;
        if let Event::Key(event) = input {
            match event.code {
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                    s.move_player(Direction::Up).map(|()| true)
                }
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                    s.move_player(Direction::Down).map(|()| true)
                }
                KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                    s.move_player(Direction::Left).map(|()| true)
                }
                KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                    s.move_player(Direction::Right).map(|()| true)
                }
                KeyCode::Esc | KeyCode::Char('q') => Ok(false),
                _ => Ok(true),
            }
        } else {
            Ok(true)
        }
    }
}

/// Render Stage `s` and additional message `msg`.
/// # Panics
/// (TUI) It panics when `crossterm::excute` failed.
fn render(s: &mut Stage, msg: &str) {
    let stage_string = format!("{}WASD to move, Q to quit.\n{}", s, msg);
    #[cfg(not(feature = "tui"))]
    {
        println!("{}", stage_string);
    }
    #[cfg(feature = "tui")]
    {
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(stage_string)
        )
        .unwrap();
    }
}

/// Runs the game. It loads stage and interpret command.
/// # Errors
/// It returns `Err` when:
/// * File is not found
/// * Error is propagated from `Stage`
/// * (TUI) Switch terminal screen is failed.
/// # Panics
/// It panics when `crossterm::excute` failed.
pub fn run(args: Arguments) -> Result<(), Box<dyn error::Error>> {
    //load
    let fnf_msg = format!("File {} not found.\n", args.filename);
    let mut f = File::open(args.filename).map_err(|err| {
        if matches!(err.kind(), io::ErrorKind::NotFound) {
            io::Error::new(io::ErrorKind::NotFound, fnf_msg)
        } else {
            err
        }
    })?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    //switch screen
    #[cfg(feature = "tui")]
    {
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            Clear(ClearType::All),
            SetTitle("Pusher")
        )
        .map_err(|_err| "Cannot switch screen.")?;
    }
    //setup stage
    let mut s = stage::Stage::new(&contents)?;
    let mut message = String::new();
    //update
    loop {
        render(&mut s, &message);
        message.clear();
        match update(&mut s) {
            Ok(true) => (),
            Ok(false) => break,
            Err(msg) => {
                message = if cfg!(feature = "color") {
                    format!("\x1b[0;31mError: {}\x1b[0m\n", msg)
                } else {
                    format!("Error: {}", msg)
                };
            }
        };
        if s.is_won() {
            message = if cfg!(feature = "color") {
                String::from("\x1b[0;33mYou Won!\x1b[0m\n")
            } else {
                String::from("You Won!\n")
            };
            render(&mut s, &message);
            break;
        }
    }
    #[cfg(feature = "tui")]
    {
        execute!(io::stdout(), Print("\n[Press any key to quit].\n")).unwrap();
        read().unwrap(); //Wait for input.
        execute!(io::stdout(), LeaveAlternateScreen)
            .map_err(|_err| "Cannot return to original screen.")?;
    }
    Ok(())
}
