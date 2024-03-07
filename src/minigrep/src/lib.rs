//! # minigrep
//!
//! `minigrep` is a collection of utilities to search for strings
//! in a given file. Supports case-sensitive and -insensitive searches.

use std::env;
use std::error::Error;
use std::fs;

/// Config that holds the query string, the file path to query
/// and if case is ignored or not.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Build a config from command line parameters.
    ///
    /// # Panics
    ///
    /// Cannot panic. Simply returns an error if something is amiss.
    ///
    /// # Errors
    ///
    /// Returns an error when the args iterator has to few parameters.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // Execute `IGNORE_CASE=1 cargo run --bin minigrep -- to poem.txt` to test
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

/// Run the minigrep search on the given config.
///
/// # Examples
///
/// ```
/// use std::env;
/// use std::process;
/// use minigrep::Config;
/// let mut args = vec!["<self>".to_string(), "the".to_string(), "../poem.txt".to_string()].into_iter();
/// let config = Config::build(args).unwrap();
/// minigrep::run(config).unwrap();
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Search function that is case insensitive.
/// Uses a mutable vector and a for loop.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

/// Search function that is case sensitive.
/// Uses iterators and adaptors.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
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
Pick three.";

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
