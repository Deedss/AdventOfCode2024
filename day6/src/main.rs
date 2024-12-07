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

    fn move_up(&self) -> Option<Self> {
        self.r.checked_sub(1).map(|new_r| Self {
            r: new_r,
            c: self.c,
        })
    }

    fn move_down(&self) -> Option<Self> {
        self.r.checked_add(1).map(|new_r| Self {
            r: new_r,
            c: self.c,
        })
    }

    fn move_right(&self) -> Option<Self> {
        self.c.checked_add(1).map(|new_c| Self {
            r: self.r,
            c: new_c,
        })
    }

    fn move_left(&self) -> Option<Self> {
        self.c.checked_sub(1).map(|new_c| Self {
            r: self.r,
            c: new_c,
        })
    }
}

fn make_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn location_is_inside_grid(grid: &Vec<Vec<char>>, loc: Location) -> bool {
    loc.r < grid.len() && loc.c < grid[0].len()
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
            if current_location.move_up().is_some()
                && location_is_inside_grid(grid, current_location.move_up().unwrap())
            {
                if is_obstruction(grid, current_location.move_up().unwrap()) {
                    return Some(Direction::Right);
                } else {
                    return Some(Direction::Up);
                }
            }
        }
        Direction::Down => {
            if current_location.move_down().is_some()
                && location_is_inside_grid(grid, current_location.move_down().unwrap())
            {
                if is_obstruction(grid, current_location.move_down().unwrap()) {
                    return Some(Direction::Left);
                } else {
                    return Some(Direction::Down);
                }
            }
        }
        Direction::Left => {
            if current_location.move_left().is_some()
                && location_is_inside_grid(grid, current_location.move_left().unwrap())
            {
                if is_obstruction(grid, current_location.move_left().unwrap()) {
                    return Some(Direction::Up);
                } else {
                    return Some(Direction::Left);
                }
            }
        }
        Direction::Right => {
            if current_location.move_right().is_some()
                && location_is_inside_grid(grid, current_location.move_right().unwrap())
            {
                if is_obstruction(grid, current_location.move_right().unwrap()) {
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
    let mut grid = make_grid(input);
    let mut current_location = get_start_location(&grid);
    let mut current_direction = Some(Direction::Up);
    let mut sum = 0;

    loop {
        grid[current_location.r][current_location.c] = 'X';
        current_direction = get_next_direction(current_direction.unwrap(), current_location, &grid);
        if current_direction.is_none() {
            break;
        }
        current_location = current_location.next_location(current_direction.unwrap());
    }

    for r in grid.iter() {
        for c in r.iter() {
            if *c == 'X' {
                sum += 1;
            }
        }
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
        assert_eq!(part_1(input), 41);
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
