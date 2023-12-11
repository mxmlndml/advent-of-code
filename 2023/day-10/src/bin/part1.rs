#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn is_connected(symbol: char, direction: Direction) -> bool {
    match direction {
        Direction::Top => ['|', 'F', '7'],
        Direction::Right => ['-', 'J', '7'],
        Direction::Bottom => ['|', 'L', 'J'],
        Direction::Left => ['-', 'L', 'F'],
    }
    .contains(&symbol)
}

fn get_symbol(position: (usize, usize), map: &str) -> char {
    map.lines()
        .nth(position.1)
        .expect("line out of bounds")
        .chars()
        .nth(position.0)
        .expect("char out of bounds")
}

fn find_start_connection(position: (usize, usize), map: &str) -> Direction {
    let height = map.lines().count();
    let width = map.lines().next().expect("no lines").chars().count();

    if position.1 > 0
        && is_connected(
            get_symbol((position.0, position.1 - 1), map),
            Direction::Top,
        )
    {
        Direction::Top
    } else if position.0 < width
        && is_connected(
            get_symbol((position.0 + 1, position.1), map),
            Direction::Right,
        )
    {
        Direction::Right
    } else if position.1 < height
        && is_connected(
            get_symbol((position.0, position.1 + 1), map),
            Direction::Bottom,
        )
    {
        Direction::Bottom
    } else if position.0 > 0
        && is_connected(
            get_symbol((position.0 - 1, position.1), map),
            Direction::Left,
        )
    {
        Direction::Left
    } else {
        panic!("No connection found");
    }
}

fn part1(input: &str) -> String {
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            if line.contains("S") {
                line.char_indices().find_map(
                    |(x, symbol)| {
                        if symbol == 'S' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            } else {
                None
            }
        })
        .expect("no start found");

    let mut current_direction = find_start_connection(start, input);
    let mut position = start;
    let mut length = 0_u32;

    loop {
        length += 1;

        position = match current_direction {
            Direction::Top => (position.0, position.1 - 1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Bottom => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
        };

        current_direction = match get_symbol(position, input) {
            '|' => match current_direction {
                Direction::Top => Direction::Top,
                Direction::Bottom => Direction::Bottom,
                _ => panic!("not connected"),
            },
            '-' => match current_direction {
                Direction::Right => Direction::Right,
                Direction::Left => Direction::Left,
                _ => panic!("not connected"),
            },
            'L' => match current_direction {
                Direction::Bottom => Direction::Right,
                Direction::Left => Direction::Top,
                _ => panic!("not connected"),
            },
            'J' => match current_direction {
                Direction::Right => Direction::Top,
                Direction::Bottom => Direction::Left,
                _ => panic!("not connected"),
            },
            '7' => match current_direction {
                Direction::Top => Direction::Left,
                Direction::Right => Direction::Bottom,
                _ => panic!("not connected"),
            },
            'F' => match current_direction {
                Direction::Top => Direction::Right,
                Direction::Left => Direction::Bottom,
                _ => panic!("not connected"),
            },
            'S' => break,
            _ => panic!("not connected"),
        };
    }

    (length / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        const EXAMPLE_SOLUTION: &str = "4";
        let input = include_str!("example-1.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }

    #[test]
    fn example2() {
        const EXAMPLE_SOLUTION: &str = "8";
        let input = include_str!("example-2.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
