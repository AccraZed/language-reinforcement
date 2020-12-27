use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string(config.filename)?;
    let input: Vec<i64> = str_to_i32(raw.split('\n').collect())?;
    let mut list: Vec<i64> = Vec::new();

    // Populate the list with the first 25 numbers of input
    for n in 0..25 {
        list.push(input[n]);
    }

    // Begin to check the following numbers
    let mut skip = 0;
    let mut bad = 0;
    for target in input.iter() {
        // Skip first 25 digits
        if skip < 25 {
            skip += 1;
            continue;
        }

        // See if a sum is found for target in list
        match sum_exists(&mut list, *target)? {
            true => {
                list.remove(0);
                list.push(*target);
            }
            // If there is no sum, then we found the bad apple
            false => {
                println!("Bad Apple: {}", target);
                bad = *target;
                break;
            }
        }
    }

    // Find contiguous set of numbers that add up to the bad apple
    find_set(input, bad);

    Ok(())
}

fn str_to_i32(list: Vec<&str>) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut ret_list = Vec::<i64>::new();

    for n in list.iter() {
        ret_list.push(n.parse()?)
    }

    Ok(ret_list)
}

fn sum_exists(list: &mut Vec<i64>, target: i64) -> Result<bool, Box<dyn Error>> {
    for n in list.iter() {
        if list.contains(&(target - *n)) {
            return Ok(true);
        }
    }
    Ok(false)
}

// Find the contiguous set of numbers in list that add up to target
// Print out the boundaries of that list, as well as the two numbers added together
fn find_set(list: Vec<i64>, target: i64) {
    let mut head = 0;
    let mut tail = 0;
    let mut sum = list[0];
    let list_len = list.len();

    loop {
        if head < list_len && tail < list_len {
            if sum > target {
                sum -= list[tail];
                tail += 1;
            } else if sum < target {
                head += 1;
                sum += list[head];
            } else {
                println!("Found Set! From {}..{}", list[tail], list[head]);
                find_min_max(list, tail, head);
                return;
            }
        }
    }
}

fn find_min_max(list: Vec<i64>, tail: usize, head: usize) {
    let (mut min, mut max) = (i64::MAX, 0 as i64);

    for n in list[tail..head + 1].iter() {
        if *n > max {
            max = *n;
        }
        if *n < min {
            min = *n;
        }
    }

    println!("MAX: {}\nMIN: {}\nMAX + MIN = {}", max, min, max + min);
}