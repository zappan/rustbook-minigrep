//! # Minigrep
//!
//! `minigrep` is a search utility to search for lines containing a given string
//! withing the given text. It offers case sensitive and case insensitive search.

use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
  search_for: String,
  filepath: String,
  ignore_case: bool,
}

impl Config {
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next(); // skip the path-to-executable

    let mut has_switch_i = false;
    let mut params = Vec::new();

    while let Some(arg) = args.next() {
      if arg == "-i" {
        has_switch_i = true;
      } else {
        params.push(arg);
      }
    }

    const REQUIRED_PARAMS_COUNT: usize = 2;
    if params.len() != REQUIRED_PARAMS_COUNT {
      return Err("invalid arguments");
    }

    let search_for = params.remove(0);
    let filepath = params.remove(0);
    // We don’t care about the value of the environment variable, just whether it’s set or unset,
    // so we’re checking is_ok() rather than using unwrap, expect, or any of the other methods
    let ignore_case = has_switch_i || std::env::var("IGNORE_CASE").is_ok();

    Ok(Config {
      search_for,
      filepath,
      ignore_case,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_content = fs::read_to_string(config.filepath)?;

  let results: Vec<&str> = if config.ignore_case {
    search_case_insensitive(&config.search_for, &file_content)
  } else {
    search(&config.search_for, &file_content)
  };

  for line in results {
    println!("{line}");
  }

  Ok(())
}

/// Performs a case-sensitive search, finding lines containing a given search term
/// within the text contents.
///
/// # Examples
///
/// ```
/// let result = minigrep::search("you", "\
/// Hello world
/// You wonderful world
/// the sun is shining
/// just for you
/// ");
///
/// let expected_res = vec!["just for you"];
/// assert_eq!(result, expected_res);
/// ```
pub fn search<'a>(search_for: &str, contents: &'a str) -> Vec<&'a str> {
  // // ## The initial approach with a for loop:
  // // let mut result = vec![];
  // let mut result = Vec::new();
  // for line in contents.lines() {
  //   if line.contains(search_for) {
  //     // technically, it'd be better, performance-wise, to use Vec<&str> here, but
  //     // that introduces named/explicit lifetimes, which I may want to skip temporarily
  //     result.push(line.to_string());
  //   }
  // }
  // result

  // ## Rewriting the above approach with iteartors using iterator adaptor methods:
  contents
    .lines()
    .filter(|line| line.contains(search_for))
    .collect() // collecting lines into a(noter) vector

  // ## Doing so also lets us avoid having a mutable intermediate results vector.
  // ## The functional programming style prefers to minimize the amount of mutable state to make code clearer.
  // ## Removing the mutable state might enable a future enhancement to make searching happen in parallel,
  // ## because we wouldn’t have to manage concurrent access to the results vector
}

/// Performs a case-insensitive search, finding lines containing a given search term
/// within the text contents.
///
/// # Examples
///
/// ```
/// let result = minigrep::search_case_insensitive("you", "\
/// Hello world
/// You wonderful world
/// the sun is shining
/// just for you
/// ");
///
/// let expected_res = vec!["You wonderful world", "just for you"];
/// assert_eq!(result, expected_res);
/// ```
pub fn search_case_insensitive<'a>(search_for: &str, contents: &'a str) -> Vec<&'a str> {
  // // While to_lowercase will handle basic Unicode, it won’t be 100% accurate.
  // // If we were writing a real application, we’d want to do a bit more work here...
  // let search_for_lowercased = search_for.to_lowercase();
  // let mut result = Vec::new();
  // for line in contents.lines() {
  //   if line.to_lowercase().contains(&search_for_lowercased) {
  //     result.push(line.to_string())
  //   }
  // }
  // result

  let search_for_lowercased = search_for.to_lowercase();
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&search_for_lowercased))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive_search() {
    // (Note that the backslash after the opening double quote tells Rust not to put
    // a newline character at the beginning of the contents of this string literal)
    let search_for = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(search_for, contents)
    );
  }

  #[test]
  fn case_insensitive_search() {
    let search_for = "rUSt";
    let contents = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(search_for, contents)
    );
  }
}
