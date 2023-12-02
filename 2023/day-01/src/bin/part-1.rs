fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = input.trim().split('\n').collect();

    for line in lines {
        let mut char: char;

        let mut chars = line.chars();
        loop {
            char = chars.next().unwrap();

            if char.is_digit(10) {
                sum += char.to_digit(10).unwrap() * 10;
                break;
            }
        }

        let mut chars = line.chars();
        loop {
            char = chars.next_back().unwrap();

            if char.is_digit(10) {
                sum += char.to_digit(10).unwrap();
                break;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("example-1.txt");
        assert_eq!(part1(input), "142".to_string());
    }
}
