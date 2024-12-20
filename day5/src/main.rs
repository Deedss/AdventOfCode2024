use std::collections::HashMap;

fn get_page_ordering_rules(input: &str) -> HashMap<u8, Vec<u8>> {
    let page_rules: Vec<(u8, u8)> = input
        .lines()
        .filter(|line| line.contains('|'))
        .map(|line| {
            let mut parts = line.split('|').map(|n| n.parse::<u8>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let mut page_ordering_rules: HashMap<u8, Vec<u8>> = HashMap::new();

    page_rules.iter().for_each(|&rule| {
        page_ordering_rules.insert(rule.0, vec![]);
    });

    page_rules.iter().for_each(|&rule| {
        page_ordering_rules
            .get_mut(&rule.0)
            .unwrap()
            .push(rule.1.clone())
    });

    page_ordering_rules
}

fn get_page_updates(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| {
            let parts = line.split(',');
            parts.map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>()
        })
        .collect()
}

fn is_page_update_valid(rules: &HashMap<u8, Vec<u8>>, page_update: &Vec<u8>) -> bool {
    let mut valid = true;

    for (page_idx, page) in page_update.iter().enumerate() {
        let values = rules.get(page);
        if values.is_some()
            && page_update[0..page_idx]
                .iter()
                .any(|i| values.unwrap().contains(i))
        {
            valid = false;
            break;
        }
    }

    valid
}

fn sort_pages(rules: &HashMap<u8, Vec<u8>>, pages: &Vec<u8>) -> Vec<u8> {
    let mut sorted_pages = pages.clone();
    sorted_pages.sort_by(|a, b| {
        if rules.get(a).is_some_and(|x| x.contains(b)) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    sorted_pages
}

fn part_1(input: &str) -> u64 {
    let page_ordering_rules = get_page_ordering_rules(input);
    let page_updates = get_page_updates(input);
    let mut sum = 0 as u64;
    for page in page_updates.iter() {
        let result = is_page_update_valid(&page_ordering_rules, page);
        if result {
            sum += page[page.len() / 2] as u64;
        }
    }

    sum
}

fn part_2(input: &str) -> u64 {
    let page_ordering_rules = get_page_ordering_rules(input);
    let mut invalid_page_updates = vec![];
    let page_updates = get_page_updates(input);
    let mut sum = 0;

    for page in page_updates.iter() {
        let result = is_page_update_valid(&page_ordering_rules, page);
        if !result {
            invalid_page_updates.push(page.clone());
        }
    }

    for page in invalid_page_updates.iter() {
        // fix the values
        let sorted = sort_pages(&page_ordering_rules, page);

        // Get middle value from fixed page
        sum += sorted[sorted.len() / 2] as u64;
    }

    sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_1(input), 143);
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
