use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "{}{}{}",
            "Correct Syntax:\n",
            "cargo run check <filename.txt>\n",
            "cargo run place <n queens> <m x m board>",
        );
        process::exit(1);
    }

    if let Err(e) = queens::run(args) {
        println!("run() error: {}", e);
        process::exit(1);
    }
}
