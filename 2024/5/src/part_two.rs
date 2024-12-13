use std::collections::{HashMap, HashSet, VecDeque};
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

fn fix_row(row: &[i64], elements_before: &HashMap<i64, HashSet<i64>>, elements_after: &HashMap<i64, HashSet<i64>>) -> Vec<i64>{
    let mut res: Vec<i64> = Vec::new();
    
    let mut num_elements_before: HashMap<i64, i64> = HashMap::new();
    for el in row {
        *num_elements_before.entry(*el).or_insert(0) += 0;
        if let Some(elements_before_set) = elements_before.get(el) {
            for element_before in elements_before_set {
                if !row.contains(element_before) { continue }
                *num_elements_before.entry(*el).or_insert(0) += 1;
            }
        }
    }
    
    let mut q: VecDeque<i64> = VecDeque::new();
    for (el, num_before) in num_elements_before.iter() {
        if *num_before == 0 {
            q.push_back(*el);
        }
    }
    
    while !q.is_empty() {
        let el = q.pop_front().unwrap();
        res.push(el);
        
        if !elements_after.contains_key(&el){
            continue
        }
        
        for element_after in elements_after.get(&el).unwrap(){
            *num_elements_before.entry(*element_after).or_insert(0) -= 1;
            if *num_elements_before.get(element_after).unwrap() == 0 {
                q.push_back(*element_after);
            }
        }
    }
    
    res
}

pub fn solve() -> i64 {
    let mut res: i64 = 0;

    let mut reading_rules = true;
    let mut elements_after: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut elements_before: HashMap<i64, HashSet<i64>> = HashMap::new();
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
                elements_before.entry(after).or_insert_with(HashSet::new).insert(before);
            }

            continue
        }

        let row: Vec<i64> = line.split(",")
            .map(|x| x.parse::<i64>().expect("Failed to parse number"))
            .collect();

        if !is_correct(&row, &elements_after){
            let fixed_row = fix_row(&row, &elements_before, &elements_after);
            res += get_middle(&fixed_row);
        }

    }

    res
}
