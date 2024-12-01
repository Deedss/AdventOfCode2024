fn distance_part_1(input: &str) -> u64 {
    let mut left_list: Vec<u64> = vec![];
    let mut right_list: Vec<u64> = vec![];

    input.lines().for_each(|line| {
        for (idx, c) in line.split_whitespace().enumerate() {
            if idx == 0 {
                left_list.push(c.parse::<u64>().unwrap());
            } else if idx == 1 {
                right_list.push(c.parse::<u64>().unwrap());
            }
        }
    });

    // Sort vectors
    left_list.sort();
    right_list.sort();

    assert_eq!(left_list.len(), right_list.len());

    let mut sum = 0;

    for i in 0..left_list.len() {
        if left_list.get(i) >= right_list.get(i) {
            sum += left_list.get(i).unwrap() - right_list.get(i).unwrap();
        } else {
            sum += right_list.get(i).unwrap() - left_list.get(i).unwrap();
        }
    }

    sum
}

fn sum_part_2(input: &str) -> u64 {
    let mut left_list: Vec<u64> = vec![];
    let mut right_list: Vec<u64> = vec![];

    input.lines().for_each(|line| {
        for (idx, c) in line.split_whitespace().enumerate() {
            if idx == 0 {
                left_list.push(c.parse::<u64>().unwrap());
            } else if idx == 1 {
                right_list.push(c.parse::<u64>().unwrap());
            }
        }
    });

    assert_eq!(left_list.len(), right_list.len());

    let mut sum = 0;

    for x in left_list.iter() {
        let mut count = 0;
        for y in right_list.iter() {
            if x == y {
                count += 1;
            }
        }
        sum += *x * count as u64;
    }

    sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = include_str!("example.txt");
        assert_eq!(distance_part_1(input), 11);
    }

    #[test]
    fn part_1() {
        let input = include_str!("input.txt");
        println!("calculate distance {}", distance_part_1(input));
    }

    #[test]
    fn part_2_example() {
        let input = include_str!("example.txt");
        assert_eq!(sum_part_2(input), 31);
    }

    #[test]
    fn part_2() {
        let input = include_str!("input.txt");
        println!("Sum is {}", sum_part_2(input));
    }
}
