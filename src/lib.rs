use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
  search_for: String,
  filepath: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    // As you become more experienced with Rust, it’ll be easier to start with the
    // most efficient solution, but for now, it’s perfectly acceptable to call clone.
    let search_for = args[1].clone();
    let filepath = args[2].clone();
    Ok(Config {
      search_for,
      filepath,
    })
  }
}

pub fn prelude(config: &Config) {
  print!("Searching for the string '{}'", config.search_for);
  println!(" in file {}:", config.filepath);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_content = fs::read_to_string(config.filepath)?;

  for line in search(&config.search_for, &file_content) {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    // (Note that the backslash after the opening double quote tells Rust not to put
    // a newline character at the beginning of the contents of this string literal)
    let search_for = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(search_for, contents)
    );
  }
}
