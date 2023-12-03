const EXAMPLE_SOLUTION: &str = "42";

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    "42".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = include_str!("example-1.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
