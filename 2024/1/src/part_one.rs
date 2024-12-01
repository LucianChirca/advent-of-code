use crate::utils::read_input;

pub fn solve() -> i32 {
    let (mut list_one, mut list_two) = read_input();
    list_one.sort();
    list_two.sort();

    let sum_of_differences: i32 = list_one.iter().zip(list_two.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    return sum_of_differences;
}
