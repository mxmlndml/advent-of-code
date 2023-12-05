fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, line| {
            acc + line
                .split(": ")
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

                    let count: u32 = own_numbers
                        .iter()
                        .filter(|own_number| winning_numbers.contains(own_number))
                        .count()
                        .try_into()
                        .expect("should be u32");

                    if count == 0 {
                        acc
                    } else {
                        acc + 2_u32.pow(count - 1)
                    }
                })
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION: &str = "13";

    #[test]
    fn example1() {
        let input = include_str!("example.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
