use std::fs;
use std::{error::Error};

struct Queen {
    x: u128,
    y: u128,
}

struct Board {
    width: u128,
    height: u128,
    queens: Vec<Queen>,
}

impl Queen {
    // pub fn new(x: u128, y: u128) -> Result<Queen, Box<dyn Error>> {
    //     Ok(Queen { x, y })
    // }
    pub fn new_from_coord(coord: &str) -> Result<Queen, Box<dyn Error>> {
        let mut x = 0;
        let mut index_num = 0;
        for (i, c) in coord.chars().enumerate() {
            if c.is_numeric() {
                index_num = i;
                break;
            }
            x *= 26;
            x += c as u128 - 'a' as u128 + 1;
        }

        // Prevents crash on empty strings
        if index_num == 0 {
            return Ok(Queen { x, y: 0 });
        }

        let y = coord[index_num..].parse::<u128>()?;

        Ok(Queen { x, y })
    }

    pub fn in_bounds(&self, board: &Board) -> Result<(), Box<dyn Error>> {
        if self.x > board.width || self.y > board.height || self.y == 0 {
            return Err(format!("Queen is out of bounds: {}, {}", self.x, self.y).into());
        }

        Ok(())
    }
}

impl Board {
    pub fn new(width: u128, height: u128) -> Result<Board, Box<dyn Error>> {
        let queens = Vec::new();

        Ok(Board {
            width,
            height,
            queens,
        })
    }
    pub fn new_from_file(path: &str) -> Result<Board, Box<dyn Error>> {
        let input = fs::read_to_string(path)?;
        let mut coordinates: Vec<&str> = input.split('\n').collect();
        let dimensions: Vec<&str> = coordinates.remove(0).split(' ').collect();

        let width = dimensions[0].parse::<u128>()?;
        let height = dimensions[1].parse::<u128>()?;

        let mut board = Board::new(width, height)?;

        for coord in coordinates {
            let queen = Queen::new_from_coord(coord)?;

            // Check for invalid queen
            match queen.in_bounds(&board) {
                Ok(()) => {
                    println!("Queen pushed: {}, {}", queen.x, queen.y);
                    board.queens.push(queen);
                },
                Err(e) => println!("{}", e),
            }
        }
        Ok(board)
    }
}

fn queen_check(board: Board) -> Result<bool, Box<dyn Error>> {
    // Check cols
    let mut cols = vec![false; board.width as usize];
    for queen in board.queens.iter() {
        if cols[queen.x as usize - 1] {
            println!(
                "Queen at position ({},{}) is invalid: column collision",
                queen.x, queen.y
            );
            return Ok(false);
        }
        cols[queen.x as usize - 1] = true
    }

    // Check rows
    let mut rows = vec![false; board.height as usize];
    for queen in board.queens.iter() {
        if rows[queen.y as usize - 1] {
            println!(
                "Queen at position ({},{}) is invalid: row collision",
                queen.x, queen.y
            );
            return Ok(false); 
        }
        rows[queen.y as usize - 1] = true
    }

    // Check Diagonals South-East
    let mut diags = vec![false; (board.height + board.width) as usize - 1];
    for queen in board.queens.iter() {
        if diags[(queen.x + queen.y) as usize - 2] {
            println!(
                "Queen at position ({},{}) is invalid: diagonal (SE) collision",
                queen.x, queen.y
            );
            return Ok(false);
        }
        diags[(queen.x + queen.y) as usize - 2] = true
    }

    // Check Diagonals South-West
    let mut diags = vec![false; (board.height + board.width) as usize - 1];
    for queen in board.queens.iter() {
        if diags[(board.width - queen.x + queen.y) as usize - 1] {
            println!(
                "Queen at position ({},{}) is invalid: diagonal (SW) collision",
                queen.x, queen.y
            );

            return Ok(false);
        }
        diags[(board.width - queen.x + queen.y) as usize - 1] = true
    }

    Ok(true)
}
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args[1] == "check" {
        println!(
            "Is board safe?: {}",
            queen_check(Board::new_from_file(&args[2])?)?
        );
    } else if args[1] == "place" && args.len() > 3 {
        let _num_queens = args[2].parse::<u128>()?;
        let width = args[3].parse::<u128>()?;
        let height = args[4].parse::<u128>()?;

        let _board = Board::new(width, height)?;
    }
    Ok(())
}
