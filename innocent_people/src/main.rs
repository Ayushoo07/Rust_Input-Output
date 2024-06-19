use std::io; // Importing the standard input/output library

fn main() {
    // Read a line of input from the user
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Define a vector containing all the vowel characters that we want to check for
    let vowels: Vec<char> = vec!['A', 'E', 'I', 'O', 'U', 'Y'];

    // Convert the input string into a vector of characters
    let arr: Vec<char> = input.chars().collect();

    // Variables to keep track of the first digit, the last digit, and the overall result
    let mut first_digit = true;
    let mut last_digit = 0;
    let mut ans = true;

    // Iterate through each character in the input array
    for idx in arr {
        if idx.is_alphanumeric() {
            // Check if the character is not a vowel
            if !vowels.contains(&idx) {
                // Check if the character is a numeric digit
                if idx.is_numeric() {
                    // If this is not the first digit encountered
                    if !first_digit {
                        // Calculate the sum of the last digit and the current digit
                        let sum = last_digit + idx.to_digit(10).unwrap();
                        // Update the last digit
                        last_digit = idx.to_digit(10).unwrap();
                        // Check if the sum is even
                        if sum % 2 != 0 {
                            ans = false; // If the sum is odd, set the result to false
                            break; // Exit the loop early
                        }
                    } else {
                        // If this is the first digit encountered
                        first_digit = false;
                        last_digit = idx.to_digit(10).unwrap();
                    }
                } else {
                    // If the character is a letter (non-vowel)
                    first_digit = true; // Reset the first_digit flag
                    last_digit = 0; // Reset the last_digit
                }
            } else {
                // If the character is a vowel, set the result to false and exit the loop
                ans = false;
                break;
            }
        } else {
            // If the character is neither alphanumeric nor a vowel
            first_digit = true; // Reset the first_digit flag
            last_digit = 0; // Reset the last_digit
        }
    }

    // Print the result based on the value of the ans variable
    if ans {
        println!("valid");
    } else {
        println!("invalid");
    }
}
