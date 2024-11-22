use regex::Regex;

///#### Example to using this fn
/**```rust
 * use search::search;
 * 
 * fn main() {
 *     let query = "Rust";
 *     let contents = "Rust is good language";
 *     let results = search_case_instnsitive(query, contents);
 *     assert_eq!(results, vec!["Rust"]) // -> True
 * }
 * ```
*/

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let re = Regex::new(query).unwrap();
    contents
        .lines()
        .filter(|line| re.is_match(line))
        .collect()
}

///#### Example to using this fn
/**```rust
 * use search::search_case_insensitive;
 * 
 * fn main() {
 *     let query = "RUst";
 *     let contents = "Rust is good language";
 *     let results = search_case_insensitive(query, contents);
 *     assert_eq!(results, vec!["Rust"]) // -> True
 * }
 * ```
*/

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let re = Regex::new(&format!("(?i){}", regex::escape(query))).unwrap();
    contents
        .lines()
        .filter(|line| re.is_match(line))
        .collect()
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insensitive(query, contents)
        );
    }
}