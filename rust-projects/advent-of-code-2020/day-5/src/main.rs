use std::fs;
use std::io::{BufRead, BufReader};

struct HighestID {
    id: usize,
    row: usize,
    col: usize,
}

fn main() {
    let filename = "input.txt";
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut highest_id = HighestID {
        id: 0,
        row: 0,
        col: 0,
    };
    let mut seats: [[bool; 8]; 128] = [[false; 8]; 128];

    // Throw all the passengers into the array `seats`
    for token in reader.lines() {
        let passenger = token.unwrap();
        let mut row: usize = 0;
        let mut col: usize = 0;

        for (i, c) in passenger.chars().rev().enumerate() {
            if c == 'R' {
                col += usize::pow(2, i as u32);
            } else if c == 'B' {
                row += usize::pow(2, i as u32 - 3);
            }
        }
        if row * 8 + col > highest_id.id {
            highest_id.row = row;
            highest_id.col = col;
            highest_id.id = row * 8 + col;
        }
        seats[row][col] = true;
    }

    // Print all empty seats
    for (row_num, row) in seats.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            if !*col {
                println!(
                    "Empty Seat at Row {}, Col {}: ID {}",
                    row_num,
                    col_num,
                    row_num * 8 + col_num
                );
            }
        }
    }

    // Print the highest ID
    println!(
        "The highest ID on this flight is ID {} at row {}, col {}",
        highest_id.id, highest_id.row, highest_id.col
    );
}
