fn retrieve_data(input: &str) -> Vec<Vec<u64>> {
    let mut data: Vec<Vec<u64>> = vec![];

    for line in input.lines() {
        let report: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        data.push(report);
    }

    data
}

fn lt(a: u64, b: u64) -> bool {
    a < b && b - a >= 1 && b - a <= 3
}

fn gt(a: u64, b: u64) -> bool {
    a > b && a - b >= 1 && a - b <= 3
}

fn report_is_safe(report: &Vec<u64>) -> bool {
    report // Check increasing values
        .windows(2)
        .all(|w| lt(w[0], w[1]))
        || report // check decreasing values
            .windows(2)
            .all(|w| gt(w[0], w[1]))
}

fn part_1(input: &str) -> u64 {
    let data = retrieve_data(input);
    let mut count = 0;

    for report in data.iter() {
        // Check if sequential
        if report_is_safe(report) {
            count += 1;
        }
    }

    count
}

fn part_2(input: &str) -> u64 {
    let data = retrieve_data(input);
    let mut count = 0;

    for report in data.iter() {
        // Check if report is already safe
        if report_is_safe(report) {
            count += 1;
            continue;
        }

        // Check if removing one level makes it safe
        let mut is_safe_with_dampener = false;

        for i in 0..report.len() {
            let mut modified_report = report.clone();
            modified_report.remove(i); // Remove one level

            // Check if the modified report is safe
            if report_is_safe(&modified_report) {
                is_safe_with_dampener = true;
                break;
            }
        }

        if is_safe_with_dampener {
            count += 1;
        }
    }

    count
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = include_str!("example");
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("input");
        println!("Safe Reports {}", part_1(input));
    }

    #[test]
    fn test_part_2_example() {
        let input = include_str!("example");
        assert_eq!(part_2(input), 4);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("input");
        println!("Safe Reports {}", part_2(input));
    }
}
