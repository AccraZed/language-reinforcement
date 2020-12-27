use std::process;

fn main() {
    let filename = "input.txt";

    if let Err(e) = day_4::run(filename) {
        println!("run() error: {}", e);

        process::exit(1);
    }
}