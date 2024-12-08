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

fn dfs(row: &[i64], el: &i64, visited: &mut HashSet<i64>, res: &mut Vec<i64>, elements_after: &HashMap<i64, HashSet<i64>>){
    visited.insert(*el);
    
    let empty_set = HashSet::new();
    let edges = elements_after.get(el).unwrap_or(&empty_set);
    for edge in edges {
        if visited.contains(edge) { continue }
        
        dfs(row, edge, visited, res, elements_after);
    }
    if row.contains(el) {
        res.insert(0, *el);
    }
}

fn fix_row(row: &[i64], elements_after: &HashMap<i64, HashSet<i64>>) -> Vec<i64>{
    let mut res: Vec<i64> = Vec::new();
    
    let mut visited: HashSet<i64> = HashSet::new();
    for el in row {
        if visited.contains(el){ continue }
        visited.insert(*el);
        
        dfs(row, el, &mut visited, &mut res, elements_after);
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
            let fixed_row = fix_row(&row, &elements_after);
            println!("Original: {:?} Fixed: {:?}", row, fixed_row);
            res += get_middle(&fixed_row);
        }
        
        // for el in row {
        //     println!("Node: {}, Edges: {:?}", el, elements_before.get(&el).unwrap());
        // }

    }

    res
}
