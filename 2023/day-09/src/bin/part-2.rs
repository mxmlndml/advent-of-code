fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

fn get_difference(history: &Vec<i32>) -> Vec<i32> {
    let mut history = history.iter().peekable();
    let mut difference: Vec<i32> = vec![];

    loop {
        let current = history.next().expect("no value");

        if history.peek().is_none() {
            break;
        }

        let next = history.peek().expect("no next value");

        difference.push(*next - current);
    }

    difference
}

fn part2(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, dataset| {
            let history = dataset
                .split(" ")
                .map(|value| value.parse::<i32>().expect("value not numeric"))
                .collect::<Vec<i32>>();

            let mut sequences = vec![history.clone()];

            loop {
                let new_seq = get_difference(sequences.last().expect("no sequence found"));

                if !new_seq.iter().any(|value| *value != 0) {
                    break;
                }

                sequences.push(new_seq);
            }

            let mut forecast = 0;
            sequences.iter().rev().for_each(|seq| {
                forecast = seq.first().expect("no value in sequence") - forecast;
            });

            acc + forecast
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION: &str = "2";

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        assert_eq!(part2(input), EXAMPLE_SOLUTION.to_string());
    }
}
