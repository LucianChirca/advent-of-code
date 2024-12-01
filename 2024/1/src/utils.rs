use std::io;

pub fn read_input() -> (Vec<i32>, Vec<i32>){
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
    
    return (list_one, list_two);
}