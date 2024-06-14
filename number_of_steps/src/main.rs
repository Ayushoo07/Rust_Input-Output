use std::io;

fn main() {
    // Create a mutable string to store the input number of elements
    let mut n = String::new();
    
    // Read the number of elements from standard input
    io::stdin().read_line(&mut n).unwrap();
    
    // Trim and parse the input string to an integer
    let n = n.trim().parse().expect("err");
    
    // Create mutable strings to store two lines of input for the two arrays
    let mut input1: String = String::new();
    let mut input2: String = String::new();
    
    // Read the first line of input for the first array
    io::stdin().read_line(&mut input1).unwrap();
    
    // Read the second line of input for the second array
    io::stdin().read_line(&mut input2).unwrap();
    
    // Split the first input line into whitespace-separated strings, parse them to i64, and collect into a vector
    let mut arr1: Vec<i64> = input1.split_whitespace().map(|s| s.parse::<i64>().expect("error")).collect();
    
    // Split the second input line into whitespace-separated strings, parse them to i64, and collect into a vector
    let arr2: Vec<i64> = input2.split_whitespace().map(|s| s.parse::<i64>().expect("err")).collect();
    
    // Check if the lengths of both arrays match the specified number of elements
    if arr1.len() != arr2.len() || arr2.len() != n {
        panic!("elements not equal to length");
    }
    
    // Find the minimum value in the first array
    let mut min = *arr1.iter().min().unwrap();
    
    // Initialize index and answer variables
    let mut i = 0;
    let mut ans = 0;
    
    // Loop through the arrays
    while i < n {
        // If the current element in arr1 is greater than or equal to the corresponding element in arr2
        if arr1[i] >= arr2[i] {
            // Reduce arr1[i] by arr2[i] until arr1[i] is not greater than the minimum value
            while arr1[i] > min {
                arr1[i] -= arr2[i];     
                ans += 1;          
            }
        }
        
        // If the current element in arr1 is less than the minimum value, update the minimum value
        if arr1[i] < min {
            min = arr1[i];
            i = 0; // Reset the index to start over
        }
        
        // If the current element in arr1 is not equal to the minimum value, set ans to -1 and break the loop
        if arr1[i] != min {
            ans = -1;
            break;
        }
        
        // Increment the index
        i += 1;
    }
    
    // Print the final answer
    println!("{}", ans); 
}
