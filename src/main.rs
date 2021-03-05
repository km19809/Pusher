extern crate pusher;

#[cfg(feature = "argparser")]
use clap::App;
use std::process;
/// Parses CLI arguments into struct Arguments
#[cfg(not(feature = "argparser"))]
fn parse_args(mut args: std::env::Args) -> pusher::Arguments {
    let filename = args.next().unwrap_or(String::from("stage.data"));
    pusher::Arguments { filename }
}

fn main() {
    #[cfg(not(feature = "argparser"))]
    {
        let mut args = std::env::args();
        let binary_name = args.next().unwrap_or(String::from("pusher"));

        if let Err(e) = pusher::run(parse_args(args)) {
            eprintln!("Application error: {}", e);
            eprintln!(
                "Usage: {} [STAGE_FILE]\nArgs:\n STAGE_FILE   default='stage.data'",
                binary_name
            );
            process::exit(1);
        }
    }
    #[cfg(feature = "argparser")]
    {
        let matches = App::new("pusher")
            .version("0.2.0")
            .author("Minsoo Kim <km19809@users.noreply.github.com>")
            .about("Simple clone of sokoban")
            .args_from_usage(
                "--default-stage 'Shows default stage to stdout.'
        [STAGE_FILE]     'Sets the stage file to play. default=\'stage.data\''",
            )
            .get_matches();

        let filename = String::from(matches.value_of("STAGE_FILE").unwrap_or("stage.data"));
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
        if let Err(e) = pusher::run(pusher::Arguments { filename }) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
