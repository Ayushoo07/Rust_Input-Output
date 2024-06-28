use std::io; // Importing the standard input/output library

fn main() {
    // Reading the first line of input which contains the value of n
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().expect("msg"); // Parsing the input to usize

    // Reading the second line of input which contains the array elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Splitting the input string by whitespace and parsing each element to i64, collecting them into a vector
    let mut arr: Vec<i64> = input.split_whitespace().map(|s| s.parse().expect("msg")).collect();

    // Calculating prefix sums for the array
    for i in 1..arr.len() {
        arr[i] = arr[i] + arr[i - 1];
    }

    // Checking if the length of the array matches the input n
    if n != arr.len() {
        panic!("invalid number of arguments");
    }

    // Initializing max_sum to the minimum possible value for i64
    let mut max_sum = i64::MIN;

    // Looping through each element in the array to find the maximum sum of the subarrays
    for i in 0..arr.len() {
        let mut curr_sum ;
        if i == 0 {
            // If the index is 0, current sum is the first element itself
            curr_sum = arr[i];
        } else {
            // Otherwise, it's the difference between the current prefix sum and the previous prefix sum
            curr_sum = arr[i] - arr[i - 1];
        }

        // Initialize variables to calculate sum of subarrays with increasing length
        let mut j = 2;
        let mut last_sum_idx = i;
        while (last_sum_idx + j) < arr.len() {
            // Calculate the next sum and add it to the current sum
            let next_sum = arr[last_sum_idx + j] - arr[last_sum_idx];
            curr_sum += next_sum;
            j += 1;
            last_sum_idx = last_sum_idx + j - 1;
        }

        // Update max_sum if the current sum is greater than or equal to the max_sum found so far
        if curr_sum >= max_sum {
            max_sum = curr_sum;
        }
    }

    // Print the maximum sum found
    println!("{}", max_sum);
}
