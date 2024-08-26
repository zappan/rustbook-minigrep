use std::env;
use std::process;

use minigrep::{self, Config};

const USAGE: &str = "\
Usage:
$ minigrep [-i] PATTERN FILE

Parameters:
 -i  case-insensitive search";

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    eprintln!("{USAGE}",);
    process::exit(1);
  });

  if let Err(err) = minigrep::run(config) {
    eprintln!("Application error: {}", err);
    process::exit(1);
  }
}
