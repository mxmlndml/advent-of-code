use std::cmp;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn get_matches(line: &str) -> usize {
    line.split(": ")
        .skip(1)
        .take(1)
        .map(|card| card.split(" | ").collect())
        .fold(0, |acc, card: Vec<&str>| {
            let card: Vec<Vec<u32>> = card
                .iter()
                .map(|numbers| {
                    numbers
                        .split(" ")
                        .filter_map(|number| number.parse::<u32>().ok())
                        .collect()
                })
                .collect();

            let [winning_numbers, own_numbers, ..] = &card[..] else {
                panic!("card has not two lists")
            };

            let count = own_numbers
                .iter()
                .filter(|own_number| winning_numbers.contains(own_number))
                .count();

            acc + count
        })
}

fn part1(input: &str) -> String {
    let mut instances = vec![1_u32; input.lines().count()];

    input.lines().enumerate().for_each(|(i, line)| {
        for j in 0..=get_matches(line) {
            if j == 0 {
                continue;
            }
            if i + j == instances.len() {
                break;
            }

            instances[j + i] += instances[i];
        }
    });

    instances.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION: &str = "30";

    #[test]
    fn example2() {
        let input = include_str!("example.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
