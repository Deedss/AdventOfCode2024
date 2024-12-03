use regex::Regex;

fn retrieve_data(input: &str) -> Vec<(u64, u64)> {
    let mut data: Vec<(u64, u64)> = vec![];
    let mut muls: Vec<String> = vec![];

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    for mul in re.find_iter(input) {
        muls.push(String::from(mul.as_str()));
    }

    for mul in muls.iter_mut() {
        let re2 = Regex::new(r"[0-9]{1,3},[0-9]{1,3}").unwrap();
        for digits in re2.find_iter(mul) {
            let split : Vec<&str> = digits.as_str().split(',').collect();
            data.push((split[0].parse::<u64>().unwrap(), split[1].parse::<u64>().unwrap()));
        }
    }

    data
}

fn retrieve_data_part_2(input: &str) -> Vec<u64, u64>{
    let mut data: Vec<(u64, u64)> = vec![];
    let mut muls: Vec<String> = vec![];

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    for mul in re.find_iter(input) {
        muls.push(String::from(mul.as_str()));
    }

    for mul in muls.iter_mut() {
        let re2 = Regex::new(r"[0-9]{1,3},[0-9]{1,3}").unwrap();
        for digits in re2.find_iter(mul) {
            let split : Vec<&str> = digits.as_str().split(',').collect();
            data.push((split[0].parse::<u64>().unwrap(), split[1].parse::<u64>().unwrap()));
        }
    }

    data

}

fn part_1(input: &str) -> u64 {
    let data = retrieve_data(input);

    let mut sum = 0;
    for mul in data.iter() {
        sum += mul.0 * mul.1;
    }

    sum
}

fn part_2(input: &str) -> u64 {
    let new_input = sanitize_input(input);
    let data = retrieve_data(new_input.as_str());
    let mut sum = 0;
    for mul in data.iter() {
        sum += mul.0 * mul.1;
    }

    sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(input), 161);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("input");
        println!("Sum {}", part_1(input));
    }

    #[test]
    fn test_part_2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(input), 48);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("input");
        // 99583571 too high
        println!("Sum of muls {}", part_2(input));
    }
}

