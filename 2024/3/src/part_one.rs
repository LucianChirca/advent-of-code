use std::fs;
use regex::Regex;

pub fn solve() -> i64 {
    let mut res: i64 = 0;
    
    let input = fs::read_to_string("in.txt").expect("Failed to read file");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Extract all matches
    for captures in re.captures_iter(&input) {
        let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        
        res += x*y;
    }

    res
}
