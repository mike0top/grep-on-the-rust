use search::{search, search_case_insensitive};

///# This enum `ParseResult`.
#[derive(Debug)]
pub enum ParseResult<'a> {
    Lines(Vec<&'a str>),
    Count(usize),
}

///# This is fn `parse` result `ParseResult`
pub fn parse<'a>(flags: Vec<String>, query: &str, contents: &'a str) -> ParseResult<'a> {
    let mut case_insensitive = false;
    let mut count_lines = false;

    for flag in &flags {
        match flag.as_str() {
            "-i" => case_insensitive = true,
            "-c" => count_lines = true,
            _ => {  }
        }
    }

    let results = if case_insensitive {
        search_case_insensitive(query, contents)
    } else {
        search(query, contents)
    };

    if count_lines {
        ParseResult::Count(results.len())
    } else {
        ParseResult::Lines(results)
    }
}
