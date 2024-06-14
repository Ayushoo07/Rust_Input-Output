use std::collections::HashMap; // Import HashMap from the standard library
use std::io; // Import input-output module
use std::io::prelude::*; // Import necessary traits for input-output operations

fn main() {
    let mut num_input = String::new(); // Create a mutable String to store input
    io::stdin().read_line(&mut num_input).unwrap(); // Read input from stdin
    let num_elements: usize = num_input.trim().parse().expect("Please enter a valid number"); // Parse input into usize for number of elements

    let mut elements_input = String::new(); // Create another mutable String to store input
    io::stdin().read_line(&mut elements_input).expect("Expected array of integers"); // Read next input line (array of integers)
    
    // Parse the elements_input into a vector of i32 integers
    let elements: Vec<i32> = elements_input
        .split_whitespace() // Split by whitespace
        .map(|s| s.parse::<i32>().expect("Enter a valid number")) // Parse each substring into i32
        .collect(); // Collect into a Vec<i32>

    // Ensure the number of elements matches the expected count
    if elements.len() != num_elements {
        eprintln!("The number of elements does not match the specified count.");
        return;
    }

    // Call the solve function with the parsed elements
    solve(elements);
}

// Function to solve the problem of counting favorite singers
pub fn solve(ele: Vec<i32>) {
    let mut map: HashMap<i32, i32> = HashMap::new(); // Create a HashMap to count occurrences of each singer
    let mut max_cnt = 0; // Initialize max count of songs by any singer

    // Iterate through each song's singer in the playlist
    for i in ele {
        let cnt = map.get_mut(&i); // Get mutable reference to count of songs for singer i
        
        // Match on cnt to handle existing or new singer
        match cnt {
            Some(cnt) => { // If singer i exists in the HashMap
                *cnt += 1; // Increment song count for singer i
                if max_cnt <= *cnt { // Check if current singer i has more songs than previous max
                    max_cnt = *cnt; // Update max count if current singer i has more songs
                }
            },
            None => { // If singer i does not exist in the HashMap
                map.insert(i, 1); // Insert singer i with initial song count of 1
                if max_cnt <= 1 { // Check if the new singer has more songs than previous max (initial case)
                    max_cnt = 1; // Update max count to 1 for the first singer
                }
            }
        };
    }

    let cnts: Vec<&i32> = map.values().collect(); // Collect all song counts into a vector of references
    let mut ans = 0; // Initialize counter for favorite singers

    // Iterate through song counts to count favorite singers
    for value in cnts {
        if *value == max_cnt { // Check if current singer has max songs
            ans += 1; // Increment counter for favorite singers
        }
    }

    println!("{}", ans); // Print number of favorite singers
}
