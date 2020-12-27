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

fn count_part_1(groups: Vec<&str>) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;

    for group in groups {
        let group: Vec<&str> = group.split('\n').collect();
        let mut answered: [bool; 26] = [false; 26];

        for person in group {
            for c in person.chars() {
                let answer = &mut answered[(c as u32 - 'a' as u32) as usize];

                if !*answer {
                    *answer = true;
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

fn count_part_2(groups: Vec<&str>) -> Result<u32, Box<dyn Error>> {
    let mut count = 0;

    for group in groups {
        let group: Vec<&str> = group.split('\n').collect();
        let mut group_answer: u32 = 0b11111111111111111111111111; // 26 questions, in bit form

        for person in group {
            if person == "" {
                continue;
            }
            let mut person_answer = 0;
            for c in person.chars() {
                // Add the bitshifted representation of the answer to the persons total answers
                person_answer |= 1 << (c as u32 - 'a' as u32);
            }

            // 'AND' gate the persons answers with the group answer, leaving only the answers that
            // everyone in the group answered in the end
            group_answer &= person_answer;
        }

        count += group_answer.count_ones();
    }
    Ok(count)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(config.filename)?;
    let groups: Vec<&str> = input.split("\n\n").collect();

    println!(
        "{} Answers were had among the groups",
        count_part_1(groups.clone())?
    );
    println!(
        "{} Answers were consistently had among the groups",
        count_part_2(groups.clone())?
    );

    Ok(())
}
