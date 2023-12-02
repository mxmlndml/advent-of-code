const DIGITS_AS_LETTERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = input.trim().split('\n').collect();

    for line in lines {
        dbg!(line);
        'line: for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                sum += c.to_digit(10).unwrap() * 10;
                dbg!(c);
                break;
            }

            for (j, digit) in DIGITS_AS_LETTERS.iter().enumerate() {
                if line[i..(line.len())].starts_with(digit) {
                    sum += (u32::try_from(j).ok().unwrap() + 1) * 10;
                    dbg!(digit);
                    break 'line;
                }
            }
        }

        'line: for (i, c) in line.chars().rev().enumerate() {
            if c.is_digit(10) {
                sum += c.to_digit(10).unwrap();
                dbg!(c);
                break;
            }

            for (j, digit) in DIGITS_AS_LETTERS.iter().enumerate() {
                if line[0..(line.len() - i)].ends_with(digit) {
                    sum += u32::try_from(j).ok().unwrap() + 1;
                    dbg!(digit);
                    break 'line;
                }
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
        let input = include_str!("example-2.txt");
        assert_eq!(part1(input), "281".to_string());
    }
}
