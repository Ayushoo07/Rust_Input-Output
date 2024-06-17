use std::io; // Import the standard input/output library

fn main() {
    // Create a mutable string variable to store the first input line
    let mut input1 = String::new();
    // Read a line of input from the standard input and store it in `input1`
    io::stdin().read_line(&mut input1).unwrap();
    // Parse the input string into a usize (unsigned size type) and store it in `n`
    let n: usize = input1.trim().parse().expect("msg");

    // Create another mutable string variable to store the second input line
    let mut str: String = String::new();
    // Read a line of input from the standard input and store it in `str`
    io::stdin().read_line(&mut str).unwrap();
    // Convert the string `str` to a vector of characters
    let mut char_vec: Vec<char> = str.chars().collect();

    if n != char_vec.len() {
        panic!("Incorrect Number of Elements");
    }
    // Initialize a boolean variable `res` to true
    let mut res = true;

    // Iterate over the indices of the characters in `char_vec`
    for idx in 0..char_vec.len() {
        // Check if the current character is 'H' and the previous character is also 'H'
        if idx != 0 && char_vec[idx] == 'H' && char_vec[idx] == char_vec[idx - 1] {
            // If so, set `res` to false and break the loop
            res = false;
            break;
        // If the current character is '.', replace it with 'B'
        } else if char_vec[idx] == '.' {
            char_vec[idx] = 'B';
        }
    }

    // If `res` is still true after the loop
    if res {
        // Print "YES"
        println!("YES");
        // Convert the vector of characters back to a string
        let ans: String = char_vec.iter().collect();
        // Print the modified string
        println!("{}",ans);
    } else {
        // If `res` is false, print "NO"
        println!("NO");
    }
}
