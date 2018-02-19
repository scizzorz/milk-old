extern crate clap;
extern crate git2;

use clap::{Arg, App};
use git2::Repository;

fn main() {
  // https://mattgathu.github.io/writing-cli-app-rust/
  let matches = App::new("milk-init")
    .version("0.0.1")
    .author("John Weachock <jweachock@gmail.com>")
    .about("An experiment in Rust and Git.")
    .arg(Arg::with_name("path")
         .takes_value(true)
         .index(1)
         .help("Path to initialize the repository."))
    .get_matches();

  let path = matches.value_of("path").unwrap_or(".");

  let repo = match Repository::init(path) {
    Ok(repo) => repo,
    Err(e) => panic!("Failed to initialize repository at {}: {}", path, e),
  };
}
