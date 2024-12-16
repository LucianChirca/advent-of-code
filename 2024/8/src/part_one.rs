use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn solve() -> i64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in read_to_string("in.txt").unwrap().lines() {
        grid.push(line.chars().collect());
    }
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for row in 0..rows {
        for col in 0..cols {
            let c: char = grid[row][col];
            if c == '.' { continue }
            map.entry(c)
                .or_default()
                .push((row as i64, col as i64));
        }
    }
    
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for positions in map.values() {
        let nr_positions = positions.len();
        for i in 0..nr_positions {
            for j in i+1..nr_positions {
                let pos_one = positions[i];
                let pos_two = positions[j];
                
                let x_diff = pos_one.0 - pos_two.0;
                let y_diff = pos_one.1 - pos_two.1;
                let antinode_one = (pos_one.0 + x_diff, pos_one.1 + y_diff);
                if antinode_one.0 >= 0 && antinode_one.0 < cols as i64 && antinode_one.1 >= 0 && antinode_one.1 < rows as i64 {
                    antinodes.insert(antinode_one);
                }

                let antinode_two = (pos_two.0 - x_diff, pos_two.1 - y_diff);
                if antinode_two.0 >= 0 && antinode_two.0 < cols as i64 && antinode_two.1 >= 0 && antinode_two.1 < rows as i64 {
                    antinodes.insert(antinode_two);
                }
            }
        }
    }
    
    antinodes.len() as i64
}
