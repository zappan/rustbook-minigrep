use std::env;
use std::process;

use minigrep::{self, Config};

const USAGE: &str = "\
Usage:
$ minigrep [-i] PATTERN FILE

Parameters:
 -i  case-insensitive search";

fn main() {
  // ## OLD: collect()-ing args into a vector, borrowed to the build()
  // let args: Vec<String> = env::args().collect();
  // let config = Config::build(&args).unwrap_or_else(|err| {..}

  // ## Instead, we're now moving args into the build function
  let config = Config::build(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    eprintln!("{USAGE}",);
    process::exit(1);
  });

  if let Err(err) = minigrep::run(config) {
    eprintln!("Application error: {}", err);
    process::exit(1);
  }
}
