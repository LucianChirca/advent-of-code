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
        
        let mut is_safe = is_report_safe(&levels);
        for i in 0..levels.iter().count(){
            let new_report: Vec<_> = levels[..i].iter()
                .chain(levels[i + 1..].iter())
                .cloned()
                .collect();
            is_safe |= is_report_safe(&new_report);
        }
            
        res += is_safe as i32;
    }

    res
}
