use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
  search_for: String,
  filepath: String,
  ignore_case: bool,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &str> {
    let has_switch_i = args[1] == "-i";

    let required_args_count = if has_switch_i { 4 } else { 3 };
    if args.len() < required_args_count {
      return Err("not enough arguments");
    }

    let args = if has_switch_i {
      &args[2..4]
    } else {
      &args[1..3]
    };

    // As you become more experienced with Rust, it’ll be easier to start with the
    // most efficient solution, but for now, it’s perfectly acceptable to call clone.
    let search_for = args[0].clone();
    let filepath = args[1].clone();

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

pub fn prelude(config: &Config) {
  print!("Searching for the string '{}'", config.search_for);
  println!(" in file {}:", config.filepath);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_content = fs::read_to_string(config.filepath)?;

  let results: Vec<String> = if config.ignore_case {
    search_case_insensitive(&config.search_for, &file_content)
  } else {
    search(&config.search_for, &file_content)
  };

  for line in results {
    println!("> {line}");
  }

  Ok(())
}

pub fn search(search_for: &str, contents: &str) -> Vec<String> {
  // let mut result = vec![];
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(search_for) {
      // technically, it'd be better, performance-wise, to use Vec<&str> here, but
      // that introduces named/explicit lifetimes, which I may want to skip temporarily
      result.push(line.to_string());
    }
  }
  result
}

pub fn search_case_insensitive(search_for: &str, contents: &str) -> Vec<String> {
  // While to_lowercase will handle basic Unicode, it won’t be 100% accurate.
  // If we were writing a real application, we’d want to do a bit more work here...
  let search_for_lowercased = search_for.to_lowercase();
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&search_for_lowercased) {
      result.push(line.to_string())
    }
  }
  result
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
