use std::io; // Import the standard input/output library

fn main() {
    // Create a mutable string variable to store the first input line
    let mut input1 = String::new();
    // Read a line of input from the standard input and store it in `input1`
    io::stdin().read_line(&mut input1).unwrap();
    // Parse the input string into a usize (unsigned size type) and store it in `n`
    let n: usize = input1.trim().parse().expect("msg");

    // Create another mutable string variable to store the second input line
    let mut input2 = String::new();
    // Read a line of input from the standard input and store it in `input2`
    io::stdin().read_line(&mut input2).unwrap();
    // Split the input string by whitespace, parse each substring into an i64 (64-bit integer),
    // and collect them into a vector of i64
    let arr: Vec<i64> = input2.split_whitespace().map(|s| s.parse::<i64>().expect("msg")).collect();

    // Check if the length of the vector `arr` is equal to `n`
    if arr.len() != n {
        // If not, panic with the message "Elements not equal to length"
        panic!("Elements not equal to length");
    }

    // Get the last element of the vector `arr`
    let last_ele = arr[arr.len() - 1];

    // Check if the last element is divisible by 10
    if last_ele % 10 == 0 {
        // If it is, print "Yes"
        println!("Yes");
    } else {
        // If it is not, print "No"
        println!("No");
    }
}
