use std::fs;

pub fn solve() -> i64 {
    let mut res: i64 = 0;

    let input = fs::read_to_string("in.txt").expect("Failed to read file");

    println!("{}", input);

    res
}
