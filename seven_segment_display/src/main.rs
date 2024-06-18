use std::{collections::HashMap, io}; // Import standard input/output and HashMap from collections

fn main() {
    // Read the number of test cases
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().expect("msg");

    // Create a HashMap to store the number of matchsticks required for each digit
    let mut map: HashMap<char, i64> = HashMap::new();
    map.insert('1', 2);
    map.insert('2', 5);
    map.insert('3', 5);
    map.insert('4', 4);
    map.insert('5', 5);
    map.insert('6', 6);
    map.insert('7', 3);
    map.insert('8', 7);
    map.insert('9', 6);
    map.insert('0', 6);

    // Vector to store the results
    let mut ans: Vec<i64> = Vec::new();

    // Loop over each test case
    for idx in 0..n {
        let mut cnt: i64 = 0; // Initialize counter for matchsticks
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // Read input string
        let input: Vec<char> = input.chars().collect(); // Convert input string to a vector of characters
        let mut len = input.len();

        // Adjust length if it is the last test case
        if idx + 1 == n {
            len += 1;
        }

        // Special case for input "00"
        if input.len() == 2 && input[0] == '0' {
            let no_of_matchstick = map.get(&input[0]).unwrap();
            cnt += no_of_matchstick;
        } else {
            // Count the number of matchsticks for each character in the input
            for i in 0..(len - 1) {
                let no_of_matchstick = map.get(&input[i]).unwrap();
                cnt += no_of_matchstick;
            }
        }

        // Store the count of matchsticks in the results vector
        ans.push(cnt);
    }

    // Loop over the results to generate the output strings
    for i in ans {
        if i % 2 == 0 {
            // If the count is even, print '1' i/2 times
            for _ in 0..i / 2 {
                print!("1");
            }
        } else {
            // If the count is odd, print '7' followed by '1' (i-3)/2 times
            print!("7");
            for _ in 0..(i - 3) / 2 {
                print!("1");
            }
        }
        println!(); // Print a newline after each output string
    }
}
