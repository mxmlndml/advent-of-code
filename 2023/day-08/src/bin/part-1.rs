use std::collections::HashMap;

struct Element {
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut lines = input.split("\n\n");
    let instructions = lines.next().expect("no instructions");

    let mut elements: HashMap<String, Element> = HashMap::new();
    lines.next().expect("no elements").lines().for_each(|line| {
        let element = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];

        elements.insert(
            element.to_string(),
            Element {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    });

    let mut steps = 0_u32;
    let mut current_element = "AAA".to_string();
    loop {
        if current_element == "ZZZ".to_string() {
            break;
        }

        let i = steps as usize % instructions.len();

        let element = elements
            .get(&current_element)
            .expect("next element not found");

        current_element = match instructions.chars().nth(i).expect("instruction too short") {
            'L' => (&element.left).to_string(),
            'R' => (&element.right).to_string(),
            _ => panic!("invalid instruction"),
        };

        steps += 1;
    }

    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        const EXAMPLE_SOLUTION: &str = "2";

        let input = include_str!("example-1.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }

    #[test]
    fn example2() {
        const EXAMPLE_SOLUTION: &str = "6";

        let input = include_str!("example-2.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
