use std::io;

fn is_report_safe(report: &[i32]) -> bool {
    let differences: Vec<i32> = report.windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    
    if differences.iter().any(|&d| d == 0 || d.abs() > 3) {return false}

    let all_positive = differences.iter().all(|&d| d > 0);
    let all_negative = differences.iter().all(|&d| d < 0);
    
    all_positive || all_negative
}

pub fn solve() -> i32 {
    let mut res: i32 = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        let levels: Vec<i32> = input.split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        
        res += is_report_safe(&levels) as i32;
    }

    res
}
