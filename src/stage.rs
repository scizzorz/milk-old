extern crate clap;
extern crate git2;

use clap::{Arg, App};
use git2::Repository;
use std::path::Path;

fn main() {
  let matches = App::new("milk-stage")
    .version("0.0.1")
    .author("John Weachock <jweachock@gmail.com>")
    .about("An experiment in Rust and Git.")
    .arg(Arg::with_name("files")
         .takes_value(true)
         .multiple(true)
         .index(1)
         .help("Files to stage."))
    .get_matches();

  let repo = match Repository::discover(".") {
    Ok(repo) => repo,
    Err(e) => {
      panic!("Failed to open repository: {}", e);
    },
  };

  if matches.is_present("files") {
    let files : Vec<_> = matches.values_of("files").unwrap().collect();
    let mut index = match repo.index() {
      Ok(o) => o,
      Err(e) => {
        panic!("Failed to open index: {}", e);
      },
    };

    for file in files {
      if let Err(e) = index.add_path(Path::new(file)) {
        panic!("Failed to stage {}: {}", file, e);
      }

      println!("staged: {}", file);
    }

    if let Err(e) = index.write() {
      panic!("Failed to write index: {}", e);
    }
  }
  else {
    println!("{}", matches.usage());
  }
}
