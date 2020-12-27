use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let mut list: HashSet<i32> = HashSet::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        list.insert(line.unwrap().parse().unwrap());
    }
    let list_base = list.clone();

    for n in list_base {
        match find_two(2020 - n, &list) {
            Ok((x, y)) => {
                println!(
                    "{a} + {b} + {c} = 2020\n{a} * {b} * {c} = {t}",
                    a = x,
                    b = y,
                    c = n,
                    t = x * y * n,
                );
                break;
            },
            Err(_) => continue,
        };
    }
}

fn find_two(target: i32, list: &HashSet<i32>) -> Result<(i32, i32), ()> {
    for n in list {
        if list.contains(&(target - n)) {
            return Ok((*n, target - *n));
        }
    }
    Err(())
}
