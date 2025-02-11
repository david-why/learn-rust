use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(err) = minigrep::run(config) {
        eprintln!("Error occurred: {err}");
        process::exit(1);
    }
}
