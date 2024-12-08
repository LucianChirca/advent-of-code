use std::fs;
use regex::Regex;

pub fn solve() -> i64 {
    let mut res: i64 = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let input = fs::read_to_string("in.txt").expect("Failed to read file").replace("\n", "");

    let chars: Vec<char> = input.chars().collect();
    let n = chars.clone().iter().count();
    let mut val = 0;
    let mut i = 0;
    while i < n {
        let c = chars[i];
        if c == 'd' {
            let mut word_end = i + 7;
            if &input[i..word_end] == "don't()" {
                i = word_end;
                val = -1;
                continue;
            }

            word_end = i + 4;
            if &input[i..word_end] == "do()" {
                i = word_end;
                val = 1;
                continue;
            }
        }
        
        if c == 'm' && val != -1 {
            let word_end = i + 12;
            let substr = &input[i..word_end];
            println!("{}", substr);

            for captures in re.captures_iter(substr) {
                let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

                res += x*y;
            }
        }
        i += 1;
    }

    res
}
