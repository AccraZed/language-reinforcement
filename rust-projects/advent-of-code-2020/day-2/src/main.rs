use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let mut counter = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if check_pass_part2(line.unwrap()).unwrap() {
            counter += 1;
        }
    }

    println!("{} passwords were valid", counter);
}

// Parses and checks password based on part 1 of advent's day 2
// Returns true if pass is valid
fn check_pass_part1(input: String) -> Result<bool, Box<dyn Error>> {
    let mut index_dash = 0;
    let mut index_char = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '-' {
            index_dash = i;
        }
        if c == ':' {
            index_char = i - 1;
            break;
        }
    }

    let min: i32 = input[..index_dash].parse()?;
    let max: i32 = input[index_dash + 1..index_char - 1].parse()?;
    let key: char = input.chars().nth(index_char).unwrap();
    let password = input[index_char + 3..].to_string();

    let mut count = 0;

    for c in password.chars() {
        if c == key {
            count += 1;
        }
    }

    Ok(count >= min && count <= max)
}

// Parses and checks password
// Returns true if pass is valid
fn check_pass_part2(input: String) -> Result<bool, Box<dyn Error>> {
    let mut index_dash = 0;
    let mut index_char = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '-' {
            index_dash = i;
        }
        if c == ':' {
            index_char = i - 1;
            break;
        }
    }

    let key: char = input.chars().nth(index_char).unwrap();
    let password = input[index_char + 3..].to_string();
    let first: char = password
        .chars()
        .nth(input[..index_dash].parse::<usize>()? - 1)
        .unwrap();
    let second: char = password
        .chars()
        .nth(input[index_dash + 1..index_char - 1].parse::<usize>()? - 1)
        .unwrap();

    Ok(first == key && second != key || first != key && second == key)
}
