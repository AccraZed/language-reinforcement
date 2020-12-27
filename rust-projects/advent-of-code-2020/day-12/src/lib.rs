use std::{error::Error, fs, mem::swap};

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
    let mut ship_i: f64 = 0.0;
    let mut ship_j: f64 = 0.0;
    let mut star_i: f64 = 10.0;
    let mut star_j: f64 = 1.0;

    let raw = fs::read_to_string(config.filename)?;
    let instructions: Vec<&str> = raw.split('\n').collect();

    for instruction in instructions {
        let n: f64 = instruction[1..].parse()?;

        // Match the first instruction
        match &instruction[..1] {
            "F" => {
                ship_i += n * star_i;
                ship_j += n * star_j;
            }
            "L" => match n as i32 {
                180 => {
                    star_i = -star_i;
                    star_j = -star_j;
                }
                90 => {
                    star_j = -star_j;
                    swap(&mut star_i, &mut star_j);
                }
                270 => {
                    star_i = -star_i;
                    swap(&mut star_i, &mut star_j);
                }
                _ => println!("unreachable match arm reached :C"),
            },
            "R" => match n as i32 {
                180 => {
                    star_i = -star_i;
                    star_j = -star_j;
                }
                90 => {
                    star_i = -star_i;
                    swap(&mut star_i, &mut star_j);
                }
                270 => {
                    star_j = -star_j;
                    swap(&mut star_i, &mut star_j);
                }
                _ => println!("unreachable match arm reached :C"),
            },
            "N" => star_j += n,
            "S" => star_j -= n,
            "E" => star_i += n,
            "W" => star_i -= n,
            _ => println!("other"),
        }
    }

    println!(
        "i: {}\nj: {}\ni + j: {}",
        ship_i,
        ship_j,
        ship_i.abs() + ship_j.abs()
    );
    Ok(())
}
