use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(& mut input).unwrap();
    let mut char_array : Vec<char> = input.chars().collect();
    for i in 0..char_array.len() {
        if char_array[i].is_ascii_uppercase() {
            char_array[i] = char_array[i].to_ascii_lowercase();
        } else {
            char_array[i] = char_array[i].to_ascii_uppercase();
        }
    }
    let ans : String= char_array.into_iter().collect();
    println!("{}",ans);
}