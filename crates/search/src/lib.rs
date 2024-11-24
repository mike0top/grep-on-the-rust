use regex::Regex;

///#### Example to using this fn
/**```rust
 * use search::search;
 *
 * fn main() {
 *     let query = "Rust";
 *     let contents = "Rust is good language";
 *     let results = search(query, contents, false, false, false);
 *     assert_eq!(results, vec!["Rust is good language"]) // -> True
 * }
 * ```
*/

pub fn search<'a>(query: &str, contents: &'a str, number: bool, invert_match: bool, _case: bool) -> Vec<String> {
    let mut re = Regex::new(query).unwrap();
    if _case {
        re = Regex::new(&format!("(?i){}", regex::escape(query))).unwrap();
    }
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if re.is_match(line) != invert_match {
            if number {
                results.push(format!("{}: {}", i + 1, line));
            } else {
                results.push(line.to_string());
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["Duct tape."],search(query, contents, false, false, false));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, false, false, true)
        );
    }
}