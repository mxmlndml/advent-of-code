fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION: &str = "42";

    #[test]
    fn example1() {
        let input = include_str!("example-1.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
