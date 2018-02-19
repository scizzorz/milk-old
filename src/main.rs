extern crate clap;

use clap::{Arg, App, AppSettings};
use std::io::ErrorKind;
use std::process::Command;
use std::process::Stdio;
use std::process::exit;

fn main() {
  // https://mattgathu.github.io/writing-cli-app-rust/
  let matches = App::new("milk")
    .version("0.0.1")
    .author("John Weachock <jweachock@gmail.com>")
    .about("An experiment in Rust and Git.")
    .setting(AppSettings::AllowExternalSubcommands)
    .get_matches();

  // https://github.com/matthiasbeyer/imag/blob/4dca497c75c5795e948a4e5c523c2ae6c905fcf8/bin/src/main.rs
  match matches.subcommand() {
    (subcommand, Some(scmd)) => {
      let subcommand_args : Vec<&str> = match scmd.values_of("") {
        Some(values) => values.collect(),
        None => Vec::new(),
      };

      let cmd = Command::new(format!("milk-{}", subcommand))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(&subcommand_args[..])
        .spawn()
        .and_then(|mut handle| handle.wait());

      match cmd {
        Ok(exit_status) => {
          if !exit_status.success() {
            println!("{} exited with non-zero exit code", subcommand);
            exit(exit_status.code().unwrap_or(1));
          }
        },

        Err(e) => {
          match e.kind() {
            ErrorKind::NotFound => {
              println!("No such command: 'milk-{}'", subcommand);
              exit(2);
            },

            ErrorKind::PermissionDenied => {
              println!("No permission to execute 'milk-{}'", subcommand);
              exit(1);
            },

            _ => {
              println!("Error executing 'milk-{}': {:?}", subcommand, e);
              exit(1);
            },
          }
        },
      }
    },

    _ => {},
  }
}
