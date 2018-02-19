extern crate clap;
extern crate git2;

use std::path::Path;
use clap::{Arg, App};
use git2::Cred;
use git2::Config;
use git2::FetchOptions;
use git2::RemoteCallbacks;
use git2::build::RepoBuilder;

fn main() {
  let matches = App::new("milk-clone")
    .version("0.0.1")
    .author("John Weachock <jweachock@gmail.com>")
    .about("An experiment in Rust and Git.")
    .arg(Arg::with_name("url")
         .required(true)
         .takes_value(true)
         .index(1)
         .help("Source URL to clone from."))
    .arg(Arg::with_name("path")
         .takes_value(true)
         .index(2)
         .help("Path to initialize the repository."))
    .get_matches();

  let url = matches.value_of("url").unwrap();
  let path = matches.value_of("path").unwrap_or(".");

  let cfg = match Config::open_default() {
    Ok(o) => o,
    _ => panic!("Unable to open config"),
  };

  let mut builder = RepoBuilder::new();
  let mut callbacks = RemoteCallbacks::new();
  let mut fetch_opts = FetchOptions::new();

  /* see below
  let mut cred_helper = git2::CredentialHelper::new(url);
  cred_helper.config(&cfg);
  let mut cred_error = false;
  let mut any_attempts = false;
  let mut ssh_username_requested = false;
  let mut tried_sshkey = false;
  */

  callbacks.credentials(|url, username, allowed| {
    /* from an older version of Cargo's callback credential:
       modern: https://github.com/rust-lang/cargo/blob/master/src/cargo/sources/git/utils.rs#L401-L479
       version I modified: https://github.com/rust-lang/cargo/blob/bb28e71202260180ecff658cd0fa0c7ba86d0296/src/cargo/sources/git/utils.rs#L344-L391
       but I can't manage to get the closure to work properly, so I'm leaving
       it commented out for now.
    let creds = if allowed.contains(git2::SSH_KEY) {
      let user = username.map(|s| s.to_string())
        .or_else(|| cred_helper.username.clone())
        .unwrap_or("git".to_string());
      git2::Cred::ssh_key_from_agent(user.as_str())
    } else if allowed.contains(git2::USER_PASS_PLAINTEXT) {
      git2::Cred::credential_helper(&cfg, url, username)
    } else if allowed.contains(git2::DEFAULT) {
      git2::Cred::default()
    } else {
      Err(git2::Error::from_str("no authentication available"))
    };

    cred_error = creds.is_err();
    creds
    */

    println!("url = {}", url);
    let user = match username {
      Some(s) => s,
      None => "git",
    };

    println!("user = {}", user);


    let creds = Cred::ssh_key(
      user,
      Some(Path::new("/home/weachockjr/.ssh/id_rsa.pub")),
      Path::new("/home/weachockjr/.ssh/id_rsa"),
      None,
      ).expect("Could not create credentials object");

    Ok(creds)
  });

  fetch_opts.remote_callbacks(callbacks);
  builder.fetch_options(fetch_opts);

  //let repo = match Repository::clone(url, path) {
  let repo = match builder.clone(url, Path::new(path)) {
    Ok(repo) => repo,
    Err(e) => panic!("Failed to clone repository from {} into {}: {}", url, path, e),
  };
}

pub fn credentials(
  _user: &str,
  _user_from_url: Option<&str>,
  _cred: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {

  return Cred::ssh_key_from_agent(_user);
  /*
  if let Ok(cred)
  match Cred::ssh_key_from_agent(_user) {
    Ok(cred) => return Ok(cred),
    Err(e) => Err(e),
  }

  match (env::var("GIT_HTTP_USER"), env::var("GIT_HTTP_PWD")) {
    (Ok(u), Ok(p)) => return Ok(git2::Cred::userpass_plaintext(&u, &p)),
    _ => _,
  }

  Err(git2:Error::from_str("no authentication set"))
  */
}
