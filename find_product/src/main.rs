use std::io;

fn main() {
    let mut n = String::new();
    let mut input = String::new();
    io::stdin().read_line(&mut n).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = n.trim().parse().expect("msg");
    let arr: Vec<i64> = input.split_whitespace().map(|s| s.parse().expect("msg")).collect();
    let div: i64 = 1000000000 + 7;
    let mut ans = 1;
    for i in 0..arr.len() {
        ans = (arr[i] * ans)%div;
    }
    println!("{}",ans);
}