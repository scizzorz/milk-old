extern crate clap;

use clap::{Arg, App};

fn main() {
  // https://mattgathu.github.io/writing-cli-app-rust/
  let matches = App::new("milk")
    .version("0.0.1")
    .author("John Weachock <jweachock@gmail.com>")
    .about("An experiment in Rust and Git.")
    .arg(Arg::with_name("url")
         .required(true)
         .takes_value(true)
         .index(1)
         .help("urrlrl"))
    .get_matches();
  let url = matches.value_of("url").unwrap();
  println!("{}", url);
}
