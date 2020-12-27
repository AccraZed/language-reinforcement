use std::io;

fn main() {
    loop {
        println!("Would you like to convert to F or C?\nEnter Q to quit");

        // Parse input and start the loop for each temperature
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.to_uppercase().chars().nth(0).unwrap() {
            'F' => to_temperature('F'),
            'C' => to_temperature('C'),
            'Q' => break,
            _ => continue,
        };
    }
}

fn to_temperature(temp_old: char) {
    // Determine whether the user wants to convert to F or C
    let temp_new = match temp_old {
        'F' => ("Celcius", 'F'),
        'C' => ("Fahrenheit", 'C'),
        _ => return,
    };

    // Loop through temperature inputs
    loop {
        println!(
            "Enter the temperature, in degrees {}:\nEnter Q to quit",
            temp_new.0
        );

        // Request user input
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Leave function if user requests exit
        if input.to_uppercase().chars().nth(0).unwrap() == 'Q' {
            break;
        }

        // Parse the input number and convert it
        let input: f64 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => continue,
        };
        let degrees: f64 = match temp_new.1 {
            'F' => input * 9.0 / 5.0 + 32.0,
            'C' => (input - 32.0) * 5.0 / 9.0,
            _ => continue,
        };

        // Print the output, truncated to two decimal places
        println!("Output: {:.2}°{}", degrees, temp_new.1);
    }
}
