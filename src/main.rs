extern crate pusher;

use std::env;
use std::process;

/// Parses CLI arguments into struct Arguments
fn parse_args(mut args: std::env::Args) -> pusher::Arguments {
    let filename = args.next().unwrap_or(String::from("stage.data"));
    pusher::Arguments {
        filename,
    }
}

fn main() {
    let mut args=env::args();
    let binary_name = args.next().unwrap_or(String::from("pusher"));
    
    if let Err(e) = pusher::run(parse_args(args)){
        eprintln!("Application error: {}", e);
        eprintln!("Usage: {} [STAGE_FILE]\nArgs:\n STAGE_FILE   default='stage.data'",binary_name);
        process::exit(1);
    }
}
