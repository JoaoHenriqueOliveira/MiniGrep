use std::env;
use std::process;

use minigrep::Config;
//use minigrep::Config; //for bringing the Config type from the library crate
// into the binary crate's scope

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        //* We customize our message if an error arises
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file {}\n", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
