use std::error::Error;
use std::fs;
use std::{thread, time};

pub struct Config {
    filename: String,
    speed: u64,
}
/// # Legend
/// `map:` vector grid of numbers
/// ```
///     0b00000001 => ' ', Empty
///     0b00000010 => '#', Wall
///     0b00000100 => '@', Player
///     0b00001000 => '.', Breadcrumb
///     0b00010000 => 'E', Exit
/// ```
/// `p:` player coordinates
///
/// `height:` height of the maze
///
/// `width:` width of the maze
pub struct Maze {
    map: Vec<Vec<u8>>,
    p: Player,
    height: i128,
    width: i128,
}

struct Player {
    x: i128,
    y: i128,
    valid: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err(
                "Not enough arguments. Correct Syntax is:\n\ncargo run <filename.txt> <opt: speed>",
            );
        }
        let filename = args[1].clone();
        let speed = match args[2].trim().parse() {
            Ok(speed) => speed,
            Err(_) => 1, // Default
        };

        Ok(Config { filename, speed })
    }
}

impl Maze {
    pub fn new(input: String) -> Result<Maze, &'static str> {
        let mut map: Vec<Vec<u8>> = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut cur_row: i128 = 0;
        let mut cur_col: i128 = 0;
        let mut p = match Player::new(0, 0) {
            Ok(p) => p,
            Err(_) => return Err("Error parsing player"),
        };

        for (_, c) in input.chars().enumerate() {
            match c {
                '@' => {
                    p.x = cur_col;
                    p.y = cur_row;
                    p.valid = true;
                }
                '\n' => {
                    cur_row += 1;
                    if width <= cur_col {
                        width = cur_col - 1;
                        cur_col = 0;
                    }
                    continue;
                }
                '\r' => continue,
                _ => (),
            };

            if height <= cur_row {
                height += 1;
                map.push(Vec::new());
            }

            map[cur_row as usize].push(char_to_u8(c));
            cur_col += 1;
        }

        Ok(Maze {
            map,
            p,
            height,
            width,
        })
    }
}

impl Player {
    pub fn new(x: i128, y: i128) -> Result<Player, &'static str> {
        Ok(Player { x, y, valid: false })
    }
}

fn char_to_u8(c: char) -> u8 {
    match c {
        ' ' => 0b00000001,
        '#' => 0b00000010,
        '@' => 0b00000100,
        '.' => 0b00001000,
        'E' => 0b00010000,
        _ => 0,
    }
}

fn u8_to_char(n: u8) -> char {
    match n {
        0b00000001 => ' ',
        n if (n & 0b00000010 > 0) => '#',
        n if (n & 0b00000100 > 0) => '@',
        n if (n & 0b00001000 > 0) => '.',
        0b00010000 => 'E',
        _ => 'X',
    }
}

/// Check to see if the player position is in bounds
fn in_bounds(maze: &Maze) -> bool {
    maze.p.x < maze.width
        && maze.p.y < maze.height
        && (maze.map[maze.p.y as usize][maze.p.x as usize] & (char_to_u8('#') | char_to_u8('.'))
            == 0)
}

pub fn print_maze(maze: &Maze) {
    let mut output = String::new();
    for cur_row in maze.map.iter() {
        for item in cur_row.iter() {
            output.push_str(&u8_to_char(*item).to_string());
        }
        output.push_str(&'\n'.to_string());
    }
    println!("{}", &output);
}

pub fn solve_maze(maze: &mut Maze, speed: u64) -> bool {
    let dir: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    thread::sleep(time::Duration::from_millis(speed));
    print!("{}[2J", 27 as char);
    print_maze(&maze);

    if !in_bounds(&maze) {
        return false;
    }
    // First check if player is on exit
    if maze.map[maze.p.y as usize][maze.p.x as usize] & char_to_u8('E') > 0 {
        println!("Maze Solved!");
        return true;
    }

    for i in 0..4 {
        let old_x = maze.p.x;
        let old_y = maze.p.y;

        maze.p.x += dir[i].1 as i128;
        maze.p.y += dir[i].0 as i128;

        if !in_bounds(&maze) {
            maze.p.x -= dir[i].1 as i128;
            maze.p.y -= dir[i].0 as i128;
            continue;
        }

        // Move player to new spot, and add visited attribute there too
        maze.map[maze.p.y as usize][maze.p.x as usize] |= char_to_u8('@');
        maze.map[old_y as usize][old_x as usize] ^= char_to_u8('@');
        maze.map[old_y as usize][old_x as usize] |= char_to_u8('.');

        if solve_maze(maze, speed) {
            return true;
        }

        // If code reaches here, then the direction was a failure and so return to old position
        maze.map[maze.p.y as usize][maze.p.x as usize] ^= char_to_u8('@');
        maze.map[maze.p.y as usize][maze.p.x as usize] |= char_to_u8('.');
        maze.map[old_y as usize][old_x as usize] |= char_to_u8('@');
        maze.p.x -= dir[i].1 as i128;
        maze.p.y -= dir[i].0 as i128;

        // Print maze
        thread::sleep(time::Duration::from_millis(speed));
        print!("{}[2J", 27 as char);
        print_maze(maze);
    }

    return false;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut maze = Maze::new(fs::read_to_string(config.filename)?)?;

    solve_maze(&mut maze, config.speed);
    print_maze(&maze);

    Ok(())
}
