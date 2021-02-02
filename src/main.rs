extern crate pusher;

use std::env;
use std::process;

fn main() {
    
    if let Err(e) = pusher::run(env::args()){
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
