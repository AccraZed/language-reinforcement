use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";

    let a = tree_collisions(filename, 1);
    let b = tree_collisions(filename, 3);
    let c = tree_collisions(filename, 5);
    let d = tree_collisions(filename, 7);
    let e = tree_collisions_special(filename, 1);
    
    println!("{}", a * b * c * d * e);
}

fn tree_collisions(filename: &str, n: usize) -> u128 {
    let mut counter = 0;
    let mut i = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line: Vec<char> = line.unwrap().chars().collect();

        if line[i] == '#' {
            counter += 1;
        }
        i = (i + n) % 31;
    }

    println!("Right {}, down 1: {}", n, counter);
    
    counter
}

fn tree_collisions_special(filename: &str, n: usize) -> u128 {
    let mut counter = 0;
    let mut i = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (cur_line,line) in reader.lines().enumerate() {
        let line: Vec<char> = line.unwrap().chars().collect();
        if cur_line % 2 == 1 {
            continue
        }
        if line[i] == '#' {
            counter += 1;
        }
        i = (i + n) % 31;
    }

    println!("Right {}, down 2: {}", n, counter);
    
    counter
}