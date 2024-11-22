use regex::Regex;

///#### Example to using this fn
/**```rust
 * use search::search;
 * 
 * fn main() {
 *     let query = "Rust";
 *     let contents = "Rust is good language";
 *     let results = search_case_instnsitive(query, contents, false);
 *     assert_eq!(results, vec!["Rust"]) // -> True
 * }
 * ```
*/

pub fn search<'a>(query: &str, contents: &'a str, number: bool) -> Vec<String> {
    let re = Regex::new(query).unwrap();
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if re.is_match(line) {
            if number {
                results.push(format!("{}: {}", i + 1, line));
            } else {
                results.push(line.to_string()); 
            }
        }
    }

    results
}

///#### Example to using this fn
/**```rust
 * use search::search_case_insensitive;
 * 
 * fn main() {
 *     let query = "RUst";
 *     let contents = "Rust is good language";
 *     let results = search_case_insensitive(query, contents, false);
 *     assert_eq!(results, vec!["Rust"]) // -> True
 * }
 * ```
*/

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str, number: bool) -> Vec<String> {
    let re = Regex::new(&format!("(?i){}", regex::escape(query))).unwrap();
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if re.is_match(line) {
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents, false));
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
            search_case_insensitive(query, contents, false)
        );
    }
}