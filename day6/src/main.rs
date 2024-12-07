#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Location {
    r: usize,
    c: usize,
}

impl Location {
    fn next_location(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self {
                r: self.r - 1,
                c: self.c,
            },
            Direction::Down => Self {
                r: self.r + 1,
                c: self.c,
            },
            Direction::Left => Self {
                r: self.r,
                c: self.c - 1,
            },
            Direction::Right => Self {
                r: self.r,
                c: self.c + 1,
            },
        }
    }
}

fn make_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn location_is_inside_grid(grid: &Vec<Vec<char>>, loc: Location) -> bool {
    loc.r >= 0 && loc.r < grid.len() && loc.c >= 0 && loc.c < grid[0].len()
}

fn is_obstruction(grid: &Vec<Vec<char>>, loc: Location) -> bool {
    grid[loc.r][loc.c] == '#'
}

fn get_next_direction(
    current_direction: Direction,
    current_location: Location,
    grid: &Vec<Vec<char>>,
) -> Option<Direction> {
    match current_direction {
        Direction::Up => {
            let new_loc = Location {
                r: current_location.r - 1,
                c: current_location.c,
            };
            if location_is_inside_grid(grid, new_loc) {
                if is_obstruction(grid, new_loc) {
                    return Some(Direction::Right);
                } else {
                    return Some(Direction::Up);
                }
            }
        }
        Direction::Down => {
            let new_loc = Location {
                r: current_location.r + 1,
                c: current_location.c,
            };
            if location_is_inside_grid(grid, new_loc) {
                if is_obstruction(grid, new_loc) {
                    return Some(Direction::Left);
                } else {
                    return Some(Direction::Down);
                }
            }
        }
        Direction::Left => {
            let new_loc = Location {
                r: current_location.r,
                c: current_location.c - 1,
            };
            if location_is_inside_grid(grid, new_loc) {
                if is_obstruction(grid, new_loc) {
                    return Some(Direction::Up);
                } else {
                    return Some(Direction::Left);
                }
            }
        }
        Direction::Right => {
            let new_loc = Location {
                r: current_location.r,
                c: current_location.c + 1,
            };
            if location_is_inside_grid(grid, new_loc) {
                if is_obstruction(grid, new_loc) {
                    return Some(Direction::Down);
                } else {
                    return Some(Direction::Right);
                }
            }
        }
    }
    None
}

fn get_start_location(grid: &Vec<Vec<char>>) -> Location {
    let mut start = Location { r: 0, c: 0 };
    for (r, v) in grid.iter().enumerate() {
        for (c, s) in v.iter().enumerate() {
            if *s == '^' {
                start.r = r;
                start.c = c;
            }
        }
    }
    start
}

fn part_1(input: &str) -> u64 {
    let grid = make_grid(input);
    let mut current_location = get_start_location(&grid);
    let mut current_direction = Some(Direction::Up);
    let mut sum = 0;

    loop {
        current_direction = get_next_direction(current_direction.unwrap(), current_location, &grid);
        if current_direction.is_none() {
            break;
        }

        println!("Location: {:?}", current_location);
        current_location = current_location.next_location(current_direction.unwrap());
        sum += 1;
    }

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
