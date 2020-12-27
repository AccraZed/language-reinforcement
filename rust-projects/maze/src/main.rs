use std::env;
use std::process;

use maze::Config;

fn main() {
    // Prompt the user to enter the file
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = maze::run(config) {
        println!("run() error: {}", e);

        process::exit(1);
    }
}