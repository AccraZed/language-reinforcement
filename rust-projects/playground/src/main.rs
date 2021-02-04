use std::io;
use std::collections::HashMap;

    pub fn length_of_longest_substring(s: String) -> i32 {
        // Hashmap holding a character and the last index it was at
        let mut characters: HashMap<char, usize> = HashMap::new();
        
        let mut index_cutoff = 0;
        let mut max: i32 = 0;
        let mut index: usize = 0;
        
        if s.len() == 0 {
            return 0;
        }
        
        for (i, c) in s.chars().enumerate() {
            match characters.insert(c, i) {
                None => (),
                Some(latest_index) => {
                    // If there's a repeat within the bounds of the cutoff,
                    // then restart the count and update the cutoff
                    if latest_index >= index_cutoff {
                        // Check for new max
                        if (i - index_cutoff) as i32 > max {
                            max = (i - index_cutoff) as i32;
                        }
                        
                        index_cutoff = latest_index + 1;
                        
                    }
                }
            }
            index = i + 1;
        }
        
        if (index - index_cutoff) as i32 > max {
            max = (index - index_cutoff) as i32;
        }
        
        max
    }

fn main() {
    println!(" {}", i32::MAX);
    println!("{}", i32::MIN);

}
