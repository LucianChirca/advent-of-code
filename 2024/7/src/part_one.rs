use std::fs::read_to_string;

fn apply_operators(operators: &Vec<char>, nums: &[i64]) -> i64 {
    let mut res: i64 = nums[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => res += nums[i + 1],
            '*' => res *= nums[i + 1],
            _ => res += 0
        }
    }

    res
}
fn can_create_value(test_value: i64, nums: &[i64], operators: &mut Vec<char>) -> bool {
    let nr_operators = operators.len();
    let max_operators = nums.len() - 1;
    if nr_operators == max_operators {
        return test_value == apply_operators(operators, nums);
    }
    
    let mut can_create = false;
    if nr_operators < max_operators {
        for op in ['+', '*'].iter() {
            operators.push(*op);
            can_create |= can_create_value(test_value, nums, operators);
            operators.pop();
        }
    }

    can_create
}

pub fn solve() -> i64 {
    let mut res: i64 = 0;

    for line in read_to_string("in.txt").unwrap().lines() {
        let parts: Vec<&str> = line.split(':').collect();

        let test_value: i64 = parts[0].parse().expect("Invalid test value");
        let nums: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|num| num.parse().expect("Invalid number"))
            .collect();

        let mut operators: Vec<char> = Vec::new();
        if can_create_value(test_value, &nums, &mut operators) {
            res += test_value;
        }
    }
    
    res
}
