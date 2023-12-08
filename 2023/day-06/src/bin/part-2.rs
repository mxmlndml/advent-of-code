fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();

    let time = lines
        .next()
        .expect("no times found")
        .split(" ")
        .fold("".to_string(), |acc, curr| {
            if curr.parse::<u64>().is_ok() {
                acc + curr
            } else {
                acc
            }
        })
        .parse::<u64>()
        .expect("not an integer");

    let distance = lines
        .next()
        .expect("no distances found")
        .split(" ")
        .fold("".to_string(), |acc, curr| {
            if curr.parse::<u64>().is_ok() {
                acc + curr
            } else {
                acc
            }
        })
        .parse::<u64>()
        .expect("not an integer");

    let mut first_win = 0;
    for hold_time in 0..(time + 1) / 2 {
        let current_distance = hold_time * (time - hold_time);

        if distance < current_distance {
            first_win = hold_time;
            break;
        }
    }
    let last_win = time - first_win;
    let combinations = last_win - first_win + 1;

    combinations.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION: &str = "71503";

    #[test]
    fn example2() {
        let input = include_str!("example.txt");
        assert_eq!(part2(input), EXAMPLE_SOLUTION.to_string());
    }
}
