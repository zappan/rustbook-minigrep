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
