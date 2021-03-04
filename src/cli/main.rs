extern crate pusher;
extern crate clap;
use std::process;
use clap::{Arg, App};

fn main() {
    let matches = App::new("pusher")
                          .version("0.2.0")
                          .author("Minsoo Kim <km19809@users.noreply.github.com>")
                          .about("Simple clone of sokoban")
                          .args_from_usage(
                              "--default-stage 'Shows default stage to stdout.'
                              [STAGE_FILE]     'Sets the stage file to play. default=\'stage.data\''")
                          .get_matches();

    let filename=String::from(matches.value_of("STAGE_FILE").unwrap_or("stage.data"));
    if matches.is_present("default-stage") {
        println!(
            "########\n\
            #+####+#\n\
            #O##.O.#\n\
            #..@...#\n\
            ########"
        );
        process::exit(0);
    }
    if let Err(e) = pusher::run(pusher::Arguments{filename}){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}