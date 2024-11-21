use std::process;
use std::env;

use crate::{config::Config,
            run::run};

mod config;
mod run;
mod flag;

///# This is main fn.
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| { // Create config and if error then exit process with code 1.
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });

    if config.flags.contains(&"-help".to_string()) { // if -help flag in flags then print helping message and exit process with code 0.
        println!("How to use: grep [OPTIONS] [QUERY] [FILE_PATH]");
        println!("-i       ignore case");
        println!("-c       count lines in query");
        println!("-help    show this message");
        println!("-version show version minigrep");
        process::exit(0);
    }

    if config.flags.contains(&"-version".to_string()) { // if -version flag in flags then print version grep and exit process with code 0.
        println!("Version grep: 0.1.2");
        process::exit(0);
    }

    if let Err(e) = run(config) { // Call fn run if error then exit process with code 1.
        eprintln!("Error App: {e}");
        process::exit(1);
    }
}
