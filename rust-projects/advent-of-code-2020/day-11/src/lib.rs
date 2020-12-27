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

struct Tile {
    tile: char,
    repeat: bool,
}

impl Tile {
    pub fn new(tile: char) -> Result<Tile, Box<dyn Error>> {
        Ok(Tile {
            tile,
            repeat: false,
        })
    }
}
fn can_sit(layout: &Vec<Vec<Tile>>, x: usize, y: usize) -> Result<bool, Box<dyn Error>> {
    Ok(true)
}
fn is_surrounded(layout: &Vec<Vec<Tile>>, x: usize, y: usize) -> Result<bool, Box<dyn Error>> {
    Ok(false)
}
fn count_seats(layout: Vec<Vec<Tile>>) {
    let mut count = 0;

    for row in layout.iter() {
        for seat in row.iter() {
            if seat.tile == '#' {
                count += 1;
            }
        }
    }

    println!("{} seats are occupied.", count);
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string(config.filename)?;
    let raw: Vec<&str> = raw.split('\n').collect();
    let mut layout = Vec::<Vec<Tile>>::new();

    // Insert input into a 2d Tile array
    for (i, row) in raw.iter().enumerate() {
        layout.push(Vec::<Tile>::new());
        for seat in row.chars() {
            layout[i].push(Tile::new(seat)?);
        }
    }

    loop {
        let mut repeating = true;

        for (row, token) in layout.iter_mut().enumerate() {
            for (col, seat) in token.iter_mut().enumerate() {
                match seat.tile {
                    'L' => {
                        if can_sit(&layout, row, col)? {
                            seat.tile = '#';
                            seat.repeat = false;
                        } else {
                            seat.repeat = true;
                        }
                    }
                    '#' => {
                        if is_surrounded(&layout, row, col)? {
                            seat.tile = 'L';
                            seat.repeat = false;
                        } else {
                            seat.repeat = true;
                        }
                    }
                    '.' => seat.repeat = true,
                    _ => {
                        println!("Unreachable match arm reached...?");
                        break;
                    }
                }
                repeating &= seat.repeat;
            }
        }

        if repeating {
            count_seats(layout);
            break;
        }
    }

    Ok(())
}
