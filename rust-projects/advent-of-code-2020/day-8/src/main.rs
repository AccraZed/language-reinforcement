use std::{error::Error, fs};

fn main() {
    let filename = "input.txt";

    let input = fs::read_to_string(filename).unwrap();

    let tasks: Vec<&str> = input.split('\n').collect();
    let mut repeat: Vec<bool> = vec![false; input.len()];
    let mut acc: i32 = 0;
    let mut iter: i32 = 0;

    loop {
        let cur_task: Vec<&str> = tasks[iter as usize].split(' ').collect();
        if repeat[iter as usize] {
            break;
        }
        repeat[iter as usize] = true;

        match cur_task[0] {
            "nop" => {
                if try_timeline(&tasks, repeat.clone(), acc, iter).unwrap() {
                    break;
                }
                iter += 1;
            }
            "jmp" => {
                if try_timeline(&tasks, repeat.clone(), acc, iter).unwrap() {
                    break;
                }
                iter += cur_task[1].parse::<i32>().unwrap()
            }
            "acc" => {
                iter += 1;
                acc += cur_task[1].parse::<i32>().unwrap();
            }
            _ => {
                println!("Error in loop");
                break;
            }
        }
    }
}

fn try_timeline(
    tasks: &Vec<&str>,
    mut repeat: Vec<bool>,
    mut acc: i32,
    mut iter: i32,
) -> Result<bool, Box<dyn Error>> {
    let tasks_len: i32 = tasks.len() as i32;
    let cur_task: Vec<&str> = tasks[iter as usize].split(' ').collect();

    match cur_task[0] {
        "jmp" => {
            iter += 1;
        }
        "nop" => iter += cur_task[1].parse::<i32>().unwrap(),
        _ => {
            return Err(format!("Error in try_timeline()...").into());
        }
    }

    loop {
        if iter == tasks_len {
            println!("Successfully terminated! Acc: {}", acc);
            return Ok(true);
        }
        let cur_task: Vec<&str> = tasks[iter as usize].split(' ').collect();
        if repeat[iter as usize] {
            return Ok(false);
        }
        repeat[iter as usize] = true;

        match cur_task[0] {
            "nop" => {
                iter += 1;
            }
            "jmp" => iter += cur_task[1].parse::<i32>().unwrap(),
            "acc" => {
                iter += 1;
                acc += cur_task[1].parse::<i32>().unwrap();
            }
            _ => {
                return Err(format!("Error in try_timeline()...").into());
            }
        }
    }
}
