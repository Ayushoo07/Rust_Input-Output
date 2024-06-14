use std::io;

fn main() {
    // Create a mutable string to store the input word
    let mut word = String::new();
    
    // Read a line of input from standard input and store it in the `word` string
    io::stdin().read_line(&mut word).unwrap();
    
    // Initialize counters for 'Z'/'z' and 'O'/'o'
    let mut cntz = 0;
    let mut cnto = 0;
    
    // Iterate over the characters in the input word with their indices
    for (_, char) in word.char_indices() {
        // If the character is 'Z' or 'z', increment the counter for 'Z'/'z'
        if char == 'Z' || char == 'z' {
            cntz += 1;
        }
        // If the character is 'O' or 'o', increment the counter for 'O'/'o'
        if char == 'O' || char == 'o' {
            cnto += 1;
        }
    }
    
    // Check if twice the number of 'Z'/'z' characters equals the number of 'O'/'o' characters
    if 2 * cntz == cnto {
        // If the condition is met, print "Yes"
        println!("Yes");
    } else {
        // If the condition is not met, print "No"
        println!("No")
    }
}
