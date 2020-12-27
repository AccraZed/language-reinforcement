use std::io;

fn main() {
    loop {
        println!("Enter n for Fib(n):");
        let mut n: String = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        fib(n);
    }
}

fn swap(a: &mut u128, b: &mut u128) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn fib(n: i64) {
    let mut fib_cur: u128 = 1;
    let mut fib_prev: u128 = 0;

    if n == 0 {
        println!("Fib (0): 0");
    }
    for i in 1..n + 1 {
        fib_prev += fib_cur;
        println!("Fib({}): {}", i, fib_cur.to_string());
        swap(&mut fib_cur, &mut fib_prev);
    }
}