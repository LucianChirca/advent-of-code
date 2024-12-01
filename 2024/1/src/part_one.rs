use std::io;

pub fn solve() -> i32 {
    let mut list_one = Vec::new();
    let mut list_two = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if let (Some(&x), Some(&y)) = (parts.get(0), parts.get(1)) {
            list_one.push(x.parse::<i32>().unwrap());
            list_two.push(y.parse::<i32>().unwrap());
        }
    }
    list_one.sort();
    list_two.sort();

    let sum_of_differences: i32 = list_one.iter().zip(list_two.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    return sum_of_differences;
}
