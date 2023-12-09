use std::collections::HashMap;

struct Element {
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

fn lcm(a: u64, b: u64) -> u64 {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    a * b / gcd(a, b)
}

fn part2(input: &str) -> String {
    let mut lines = input.split("\n\n");
    let instructions = lines.next().expect("no instructions");

    let mut elements: HashMap<String, Element> = HashMap::new();
    let mut current_elements: Vec<String> = vec![];
    lines.next().expect("no elements").lines().for_each(|line| {
        let element = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];

        if element.ends_with("A") {
            current_elements.push(element.to_string());
        }

        elements.insert(
            element.to_string(),
            Element {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    });

    current_elements
        .iter()
        .map(|current_element| {
            let mut steps = 0_u32;
            let mut current_element = current_element;

            loop {
                if current_element.ends_with("Z") {
                    break;
                }

                let i = steps as usize % instructions.len();

                let element = elements
                    .get(current_element)
                    .expect("next element not found");

                current_element = match instructions.chars().nth(i).expect("instruction too short")
                {
                    'L' => &element.left,
                    'R' => &element.right,
                    _ => panic!("invalid instruction"),
                };

                steps += 1;
            }

            steps
        })
        .fold(1_u64, |acc, step| lcm(acc, step as u64))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        const EXAMPLE_SOLUTION: &str = "2";

        let input = include_str!("example-1.txt");
        assert_eq!(part2(input), EXAMPLE_SOLUTION.to_string());
    }

    #[test]
    fn example2() {
        const EXAMPLE_SOLUTION: &str = "6";

        let input = include_str!("example-2.txt");
        assert_eq!(part2(input), EXAMPLE_SOLUTION.to_string());
    }

    #[test]
    fn example3() {
        const EXAMPLE_SOLUTION: &str = "6";

        let input = include_str!("example-3.txt");
        assert_eq!(part2(input), EXAMPLE_SOLUTION.to_string());
    }
}
