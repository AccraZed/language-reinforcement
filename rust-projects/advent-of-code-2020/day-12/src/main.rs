use day_12::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing args: {}", e);
        process::exit(1);
    });

    if let Err(e) = day_12::run(config) {
        println!("run() error: {}", e);
        process::exit(1);
    }
}
