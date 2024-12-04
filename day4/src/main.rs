fn get_char_in_dir(
    r: i32,
    c: i32,
    dr: i32,
    dc: i32,
    grid: &Vec<Vec<char>>,
    steps: usize,
) -> Option<char> {
    let mut rr = r;
    let mut cc = c;
    for _ in 0..steps {
        rr += dr;
        cc += dc;
        if rr < 0 || cc < 0 || rr >= grid.len() as i32 || cc >= grid[rr as usize].len() as i32 {
            return None;
        }
    }
    Some(grid[rr as usize][cc as usize])
}

fn check_diagonal_pattern(
    r: i32,
    c: i32,
    dx1: i32,
    dy1: i32,
    dx2: i32,
    dy2: i32,
    grid: &Vec<Vec<char>>,
) -> bool {
    let char1 = get_char_in_dir(r, c, dx1, dy1, grid, 1);
    let char2 = get_char_in_dir(r, c, dx2, dy2, grid, 1);

    (char1 == Some('M') && char2 == Some('S')) || (char1 == Some('S') && char2 == Some('M'))
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn part_1(input: &str) -> u64 {
    let directions: Vec<(i32, i32)> = vec![
        (-1, 0),  // N
        (-1, 1),  // NE
        (0, 1),   // E
        (1, 1),   // SE
        (1, 0),   // S
        (1, -1),  // SW
        (0, -1),  // W
        (-1, -1), // NW
    ];

    let grid: Vec<Vec<char>> = get_grid(input);
    let mut sum = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'X' {
                for &(dr, dc) in &directions {
                    if get_char_in_dir(r as i32, c as i32, dr, dc, &grid, 1) == Some('M')
                        && get_char_in_dir(r as i32, c as i32, dr, dc, &grid, 2) == Some('A')
                        && get_char_in_dir(r as i32, c as i32, dr, dc, &grid, 3) == Some('S')
                    {
                        sum += 1;
                    }
                }
            }
        }
    }

    sum
}

fn part_2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = get_grid(input);
    let mut sum = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 'A' {
                let r = r as i32;
                let c = c as i32;

                // Check diagonal patterns
                let match_1 = check_diagonal_pattern(r, c, 1, 1, -1, -1, &grid);
                let match_2 = check_diagonal_pattern(r, c, -1, 1, 1, -1, &grid);

                if match_1 && match_2 {
                    sum += 1;
                }
            }
        }
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
        assert_eq!(part_1(input), 18);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("input.txt");
        println!("Sum {}", part_1(input));
    }

    #[test]
    fn test_part_2_example() {
        let input = include_str!("example2.txt");
        assert_eq!(part_2(input), 9);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("input.txt");
        println!("Sum {}", part_2(input));
    }
}
