use std::env;
use std::process;

use day_7::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = day_7::run(config) {
        println!("run() error: {}", e);
        process::exit(1);
    }
}
