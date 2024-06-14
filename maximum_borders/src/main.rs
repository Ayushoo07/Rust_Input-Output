use std::{cmp::max, io::{self, BufRead}};

fn main() {
    // Get the standard input handle
    let stdinput = io::stdin();
    // Lock the standard input and create an iterator over lines
    let mut test_cases = stdinput.lock().lines();

    // Read the number of test cases from the first line of input
    let n: usize = test_cases.next().unwrap().unwrap().trim().parse().expect("Please enter a valid number");

    // Vector to hold all the matrices from the input
    let mut all_matrices: Vec<Vec<String>> = Vec::new();

    // Loop over each test case
    for _ in 0..n {
        // Read the dimensions of the matrix (rows and columns)
        let dimensions = test_cases.next().unwrap().unwrap();
        let dimensions: Vec<usize> = dimensions.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (rows, cols) = (dimensions[0], dimensions[1]);

        // Vector to hold the current matrix
        let mut matrix: Vec<String> = Vec::new();

        // Loop to read each row of the matrix
        for _ in 0..rows {
            let row = test_cases.next().unwrap().unwrap();
            // Ensure the row length matches the expected number of columns
            if row.len() != cols {
                panic!("Invalid number of arguments");
            }
            matrix.push(row);
        }
        // Add the current matrix to the list of all matrices
        all_matrices.push(matrix);
    }

    // Process each matrix to find the maximum count of consecutive '#'
    for matrix in all_matrices {
        let mut max_cnt_row = 0; // Max consecutive '#' in any row
        let mut max_cnt_col = 0; // Max consecutive '#' in any column
        let mut col_helper: Vec<i32> = Vec::new(); // Helper vector for column-wise counts

        // Loop over each row in the matrix
        for row in 0..matrix.len() {
            let mut cnt_col = 0; // Counter for current column
            let mut cnt_row = 0; // Counter for current row

            // Loop over each character in the row with its index
            for (idx, char) in matrix[row].char_indices() {
                // Column-wise processing
                if row == 0 && char == '#' {
                    // Initialize the column helper vector for the first row
                    col_helper.push(1);
                    max_cnt_col = max(max_cnt_col, 1);
                } else if row == 0 {
                    col_helper.push(0);
                } else if char == '#' {
                    // Update the helper vector and max count for columns
                    col_helper[idx] += 1;
                    max_cnt_col = max(max_cnt_col, col_helper[idx]);
                } else {
                    col_helper[idx] = 0;
                }

                // Row-wise processing
                if char == '#' && idx == 0 {
                    cnt_row += 1;
                    max_cnt_row = max(max_cnt_row, cnt_row);
                } else if cnt_row != 0 && char == '#' {
                    cnt_row += 1;
                    max_cnt_row = max(max_cnt_row, cnt_row);
                } else if char == '#' {
                    cnt_row = 1;
                    max_cnt_row = max(max_cnt_row, cnt_row);
                } else {
                    cnt_row = 0;
                }
            }
        }
        // Print the maximum of the row-wise and column-wise max counts
        println!("{}", max(max_cnt_col, max_cnt_row));
    }
}
