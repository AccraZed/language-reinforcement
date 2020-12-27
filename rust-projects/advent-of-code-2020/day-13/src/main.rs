use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = day_13::Config::new(args[1].clone()).unwrap_or_else(|e| {
        println!("Error creating config: {}", e);
        process::exit(1);
    });

    if let Err(e) = day_13::run(config) {
        println!("Error in run(): {}", e);
        process::exit(1);
    }
}
