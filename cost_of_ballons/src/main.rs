use std::io::{self, BufRead}; // Import standard input/output library with BufRead for buffered reading

fn main() {
    let stdin = io::stdin(); // Get the standard input handle
    let mut testcases = stdin.lock().lines(); // Lock the standard input for reading lines
    
    // Read the first line, unwrap the result, trim whitespace, parse it as a usize, and assign to `n`
    let n: usize = testcases.next().unwrap().unwrap().trim().parse().expect("msg");

    // Loop over the number of test cases
    for _ in 0..n {
        // Read the next line for the costs, unwrap it, split it by whitespace,
        // parse each part as i64, and collect into a vector of i64
        let cost = testcases.next().unwrap().unwrap();
        let mut cost: Vec<i64> = cost.split_whitespace().map(|s| s.parse::<i64>().expect("msg")).collect();
        
        let mut total_cost: i64 = 0; // Initialize total cost to 0
        let mut first_problem = 0; // Initialize counter for the first problem
        let mut second_problem = 0; // Initialize counter for the second problem
        
        cost.sort(); // Sort the costs in ascending order
        
        // Read the next line for the number of participants, unwrap it, trim whitespace,
        // parse it as a usize, and assign to `participants`
        let participants = testcases.next().unwrap().unwrap();
        let participants: usize = participants.trim().parse().expect("msg");
        
        // Loop over the number of participants
        for _ in 0..participants {
            // Read the next line for the results, unwrap it, split it by whitespace,
            // parse each part as i64, and collect into a vector of i64
            let result: Vec<i64> = testcases.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<i64>().expect("msg")).collect();
            
            // Accumulate the results for the first and second problems
            for k in 0..2 {
                if k == 0 {
                    first_problem += result[k];
                } else {
                    second_problem += result[k];
                }
            }
        }
        
        // Calculate the total cost based on the accumulated results
        if first_problem > second_problem {
            // If more participants solved the first problem, multiply by the lower cost
            total_cost = first_problem * cost[0] + second_problem * cost[1];
        } else {
            // If more participants solved the second problem, multiply by the lower cost
            total_cost = first_problem * cost[1] + second_problem * cost[0];
        }
        
        // Print the total cost for this test case
        println!("{}", total_cost);
    }
}
