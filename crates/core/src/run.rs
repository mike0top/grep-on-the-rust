use std::error::Error;
use std::fs;

use crate::config::Config;
use crate::flag::{parse, ParseResult};

///# This is fn run.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = parse(config.flags, config.query.as_str(), contents.as_str());

    match results {
        ParseResult::Lines(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        ParseResult::Count(count) => {
            println!("{}", count);
        }
    }

    Ok(())
}