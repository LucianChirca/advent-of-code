use std::collections::HashMap;
use crate::utils::read_input;

pub fn solve() -> i32 {
    let (list_one, list_two) = read_input();

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &num in &list_two {
        *counts.entry(num).or_insert(0) += 1;
    }
    
    return list_one.iter()
                    .map(
                        |num|
                        num * (*counts.get(num).unwrap_or(&0))
                    ).sum();
}