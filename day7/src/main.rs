use std::collections::HashMap;

fn parse_values(input: &str) -> HashMap<u64, Vec<u64>> {
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        let mut parts = line.trim().split(':');
        let sum = parts.next().unwrap().parse::<u64>().unwrap();
        let values = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        map.insert(sum, values);
    });
    map
}

fn evaluate(values: &Vec<u64>, operators: &Vec<char>) -> u64 {
    let mut result = values[0];
    for (idx, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += values[idx + 1],
            '*' => result *= values[idx + 1],
            _ => unreachable!(),
        }
    }
    result
}

fn is_valid_equation(sum: u64, values: &Vec<u64>) -> bool {
    let mut is_valid = false;

    let operator_count = values.len() - 1;
    let mut operators = vec!['+'; operator_count];

    loop {
        if evaluate(values, &operators) == sum {
            is_valid = true;
            break;
        }
        // Generate the next operator sequence
        let mut carry = true;
        for i in (0..operator_count).rev() {
            if carry {
                if operators[i] == '+' {
                    operators[i] = '*';
                    carry = false;
                } else {
                    operators[i] = '+';
                    carry = true;
                }
            }
        }

        if carry {
            break; // All combinations exhausted
        }
    }

    is_valid
}

fn part_1(input: &str) -> u64 {
    let values = parse_values(input);
    let mut sum = 0;

    values.iter().for_each(|(&k, v)| {
        if is_valid_equation(k, v) {
            sum += k;
        }
    });

    sum
}

fn part_2(input: &str) -> u64 {
    let mut sum = 0;

    sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = include_str!("example.txt");
        println!("Sum {}", part_1(input));
        // assert_eq!(part_1(input), 41);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("input.txt");
        println!("Sum {}", part_1(input));
    }

    #[test]
    fn test_part_2_example() {
        let input = include_str!("example.txt");
        println!("Sum {}", part_2(input));
        // assert_eq!(part_2(input), 123);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("input.txt");
        println!("Sum {}", part_2(input));
    }
}
