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

fn part_1(input: &str) -> u64 {
    let data = retrieve_data(input);
    let mut count = 0;

    for report in data.iter() {
        // Check if sequential
        if report // Check increasing values
            .windows(2)
            .all(|w| w[0] < w[1] && w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
            || report // check decreasing values
                .windows(2)
                .all(|w| w[0] > w[1] && w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
        {
            count += 1;
        }
    }

    count
}

fn part_2(input: &str) -> u64 {
    let data = retrieve_data(input);
    let mut unsafe_reports = vec![];
    let mut count = 0;

    // Count safe reports
    for report in data.iter() {
        if report // Check increasing values
            .windows(2)
            .all(|w| w[0] < w[1] && w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
            || report // check decreasing values
                .windows(2)
                .all(|w| w[0] > w[1] && w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
        {
            count += 1;
        } else {
            unsafe_reports.push(report);
        }
    }

    // Go over unsafe reports
    for report in unsafe_reports.iter() {
        if report.windows(3).all(|w| {
            (w[0] < w[1] && w[1] - w[0] < 1 && w[1] - w[0] > 3)
                && (w[0] < w[2] && w[2] - w[0] >= 1 && w[2] - w[0] <= 3)
        }) {
            count += 1;
        } else if report.windows(3).all(|w| {
            (w[0] > w[1] && w[0] - w[1] < 1 && w[0] - w[1] > 3)
                && (w[0] > w[2] && w[2] - w[0] >= 1 && w[2] - w[0] <= 3)
        }) {
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
        // 642 is too high
        println!("Safe Reports {}", part_2(input));
    }
}
