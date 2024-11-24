use search::search;

///# This enum `ParseResult`.
#[derive(Debug)]
pub enum ParseResult {
    Lines(Vec<String>),
    Count(usize),
}

///# This is fn `parse` result `ParseResult`
pub fn parse<'a>(flags: Vec<String>, query: &str, contents: &'a str) -> ParseResult {
    let mut case_insensitive = false;
    let mut count_lines = false;
    let mut numbers_lines = false;
    let mut invert_match = false;

    for flag in &flags {
        match flag.as_str() {
            "-i" => case_insensitive = true,
            "-c" => count_lines = true,
            "-n" => numbers_lines = true,
            "-v" => invert_match = true,
            _ => {  }
        }
    }

    let results = search(query, contents, numbers_lines, invert_match, case_insensitive);


    if count_lines {
        ParseResult::Count(results.len())
    } else {
        ParseResult::Lines(results)
    }
}
