use std::{error::Error, fs};
pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

fn str_to_int(list: &Vec<&str>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut ret_list: Vec<i32> = Vec::new();
    for token in list {
        ret_list.push(token.parse()?)
    }

    Ok(ret_list)
}

fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

fn merge_sort<T: Copy + Ord>(x: &mut [T]) {
    let n = x.len();
    let m = n / 2;

    if n <= 1 {
        return;
    }

    merge_sort(&mut x[0..m]);
    merge_sort(&mut x[m..n]);

    let mut y: Vec<T> = x.to_vec();

    merge(&x[0..m], &x[m..n], &mut y[..]);

    x.copy_from_slice(&y);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string(config.filename)?;
    let mut list = str_to_int(&raw.split('\n').collect::<Vec<&str>>())?;

    merge_sort(&mut list);
    
    let (mut one, mut two, mut three) = (0, 0, 0);
    let mut prev = 0;

    for cur in list {
        match cur - prev {
            1 => one += 1,
            2 => two += 1,
            3 => three += 1,
            _ => (), // Unreachable
        };
        prev = cur;
    }
    three += 1;
    
    println!("Ones: {}\nTwos: {}\nThrees: {}\nOnes * Threes: {}", one, two, three, one * three);

    Ok(())
}
