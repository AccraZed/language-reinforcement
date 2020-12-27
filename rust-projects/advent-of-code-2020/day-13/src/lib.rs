use std::error::Error;
use std::fs;
const X: i64 = i64::MAX;
pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(filename: String) -> Result<Config, Box<dyn Error>> {
        Ok(Config { filename })
    }
}

struct Bus {
    id: i64,
    offset: i64,
}

fn check_valid_timestamp(busses: &[Bus], cur: i64) -> Result<Option<i64>, Box<dyn Error>> {
    // Base case
    if busses.len() == 1 {
        if cur % busses[0].id == 0 {
            return Ok(Some(cur));
        } else {
            return Ok(None);
        }
    }

    if cur % busses[0].id == 0
        && check_valid_timestamp(&busses[1..], 1 + cur + busses[0].offset)?.is_some()
    {
        return Ok(Some(cur));
    }

    Ok(None)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string(config.filename)?;
    let raw: Vec<&str> = raw.split('\n').collect();

    let earliest_depart: i64 = raw[0].parse()?;
    let bus_ids: Vec<i64> = raw[1]
        .split(',')
        .map(|s| if s == "x" { X } else { s.parse().unwrap() })
        .collect();

    let mut earliest_bus = i64::MAX;
    let mut earliest_bus_id = i64::MAX;

    // Part 1
    for id in bus_ids.iter() {
        if id - (earliest_depart % id) < earliest_bus {
            earliest_bus = id - (earliest_depart % id);
            earliest_bus_id = *id;
        }
    }

    println!(
        "earliest_bus_id: {}\nearliest_bus: {}\nmultiplied: {}\n",
        earliest_bus_id,
        earliest_bus,
        earliest_bus_id * earliest_bus
    );

    // Part 2
    let mut busses: Vec<Bus> = Vec::new();
    let mut cur_id = 0;
    let mut cur_offset = 0;

    for (i, id) in bus_ids.iter().enumerate() {
        if i == 0 {
            cur_id = *id;
            continue;
        }
        if *id == X {
            cur_offset += 1;
        } else {
            busses.push(Bus {
                id: cur_id,
                offset: cur_offset,
            });
            cur_offset = 0;
            cur_id = *id;
        }
    }
    // Push the last bus
    busses.push(Bus {
        id: cur_id,
        offset: cur_offset,
    });

    let mut i = 0;
    loop {
        let cur = check_valid_timestamp(&busses, i)?;

        if cur.is_some() {
            println!("earliest timestamp list: {}", cur.unwrap());
            break;
        }
        i += 7;
    }
    Ok(())
}
