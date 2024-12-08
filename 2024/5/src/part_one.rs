use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn is_correct(row: &[i64], elements_after: &HashMap<i64, HashSet<i64>>) -> bool{
    let n = row.len();
    
    for i in 0..n {
        let el = row[i];
        for j in i + 1..n {
            let following_el = row[j];
            if !elements_after.contains_key(&el) || !elements_after.get(&el).unwrap().contains(&following_el){
                return false;
            }
        }
    }
    
    true
}

fn get_middle(row: &[i64]) -> i64 {
    let middle_index = row.len() / 2;
    
    row[middle_index]
}

pub fn solve() -> i64 {
    let mut res: i64 = 0;

    let mut reading_rules = true;
    let mut elements_after: HashMap<i64, HashSet<i64>> = HashMap::new();
    for line in read_to_string("in.txt").unwrap().lines() {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }
        
        if reading_rules {
            let mut parts = line.split("|")
                .map(|x| x.parse::<i64>().expect("Failed to parse number"));

            if let (Some(before), Some(after)) = (parts.next(), parts.next()) {
                elements_after.entry(before).or_insert_with(HashSet::new).insert(after);
            }
            
            continue
        }
        
        let row: Vec<i64> = line.split(",")
            .map(|x| x.parse::<i64>().expect("Failed to parse number"))
            .collect();
        
        if is_correct(&row, &elements_after){
            res += get_middle(&row);
        }
        
    }
    
    res
}
