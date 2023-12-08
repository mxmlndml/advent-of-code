fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();

    let times = lines
        .next()
        .expect("no times found")
        .split(" ")
        .filter_map(|time| (*time).parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let distances = lines
        .next()
        .expect("no distances found")
        .split(" ")
        .filter_map(|time| (*time).parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let mut product = 1_u32;

    times.iter().enumerate().for_each(|(i, time)| {
        let mut first_win = 0;

        for hold_time in 0..(time + 1) / 2 {
            let distance = hold_time * (time - hold_time);

            if distances[i] < distance {
                first_win = hold_time;
                break;
            }
        }
        let last_win = time - first_win;
        let combinations = last_win - first_win + 1;
        product *= combinations;
    });

    product.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION: &str = "288";

    #[test]
    fn example1() {
        let input = include_str!("example.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
