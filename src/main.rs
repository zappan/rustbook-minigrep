use std::env;
use std::process;

use minigrep::{self, Config};

const USAGE: &str = "Usage:\n$ minigrep PATTERN FILE";

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    println!("{USAGE}",);
    process::exit(1);
  });

  minigrep::prelude(&config);
  if let Err(err) = minigrep::run(config) {
    println!("Application error: {}", err);
    process::exit(1);
  }
}
