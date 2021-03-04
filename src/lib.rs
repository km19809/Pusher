use std::error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
pub mod stage;
pub mod vector2;

/// Struct for contain parsed arguments.
pub struct Arguments {
    /// File name of stage data
    pub filename: String,
}

/// Runs the game. It loads stage and interpret command.
pub fn run(args: Arguments) -> Result<(), Box<dyn error::Error>> {
    //load
    let fnf_msg = format!(
        "File {} not found.\n",
        args.filename
    );
    let mut f = File::open(args.filename).map_err(|err| {
        if matches!(err.kind(), io::ErrorKind::NotFound) {
            io::Error::new(io::ErrorKind::NotFound, fnf_msg)
        } else {
            err
        }
    })?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    //set stage
    let mut s = stage::Stage::new(&contents)?;
    //update

    loop {
        println!("{}", s);
        if s.is_won() {
            println!("You won!");
            break;
        }
        println!("input command(WASD to move, Q to quit):");
        let mut inputs = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut inputs)
            .expect("Failed to read input.");
        let input = inputs.trim().chars().next();
        let result = match input {
            Some(cmd) => match cmd {
                'W' | 'w' => s.move_player(stage::Direction::Up),
                'A' | 'a' => s.move_player(stage::Direction::Left),
                'S' | 's' => s.move_player(stage::Direction::Down),
                'D' | 'd' => s.move_player(stage::Direction::Right),
                'Q' | 'q' => {
                    break;
                }
                _ => Err("Invalid input."),
            },
            None => Err("Empty input."),
        };
        if let Err(msg) = result {
            println!("Error: {}", msg);
        }
    }
    Ok(())
}
