const INPUT: &'static str = include_str!("input.txt");

fn condition(input: &Vec<i32>) -> bool {
    if (input.windows(2).all(|v| v[0] > v[1]) || input.windows(2).all(|v| v[0] < v[1]))
        && input
            .windows(2)
            .all(|v| (v[0] - v[1]).abs() > 0 && (v[0] - v[1]).abs() < 4)
    {
        return true;
    };
    false
}

fn part_1(reports: &Vec<Vec<i32>>) {
    let safe_reports = reports
        .iter()
        .filter(|v| {
            if condition(v) {
                return true;
            };
            return false;
        })
        .collect::<Vec<_>>();
    println!("safe reports: {}", safe_reports.len());
}

fn part_2(reports: &Vec<Vec<i32>>) {
    let safe_reports = reports
        .iter()
        .filter(|v| {
            if condition(v) {
                return true;
            };
            // part 2
            // check if removal of one element changes anything
            for n in 0..v.len() {
                let mut temp = v.to_vec();
                temp.remove(n);
                if condition(&temp) {
                    return true;
                };
            }
            return false;
        })
        .collect::<Vec<_>>();
    println!("safe reports with problem dampener: {}", safe_reports.len());
}

fn main() {
    let input_lines = INPUT.lines().collect::<Vec<_>>();
    let reports = input_lines
        .iter()
        .map(|v| {
            v.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    part_1(&reports);
    part_2(&reports);
}
