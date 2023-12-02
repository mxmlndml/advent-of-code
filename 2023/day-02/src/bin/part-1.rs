#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct LineData {
    id: u32,
    is_possible: bool,
}

const EXAMPLE_SOLUTION: &str = "8";
const CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn parse_line(line: &str) -> LineData {
    let mut line_data = LineData {
        id: 0,
        is_possible: true,
    };

    if let [id, subsets] = line.split(": ").collect::<Vec<&str>>()[..] {
        let id = id.chars().skip(5).collect::<Vec<char>>();
        let id = id.iter().collect::<String>().parse::<u32>().unwrap();
        line_data.id = id;

        subsets.split("; ").for_each(|subset: &str| {
            let turn = subset.split(", ").collect::<Vec<&str>>();
            let mut cubes: Cubes = Cubes {
                red: 0,
                green: 0,
                blue: 0,
            };
            turn.iter().for_each(|group: &&str| {
                if let [amount, color] = group.split(" ").collect::<Vec<&str>>()[..] {
                    let amount = amount.parse::<u32>().unwrap();

                    match color {
                        "red" => cubes.red = amount,
                        "green" => cubes.green = amount,
                        "blue" => cubes.blue = amount,
                        _ => panic!(),
                    }
                }
            });

            if cubes.red > CUBES.red || cubes.green > CUBES.green || cubes.blue > CUBES.blue {
                line_data.is_possible = false;
                return;
            }
        });
    }

    line_data
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    lines
        .iter()
        .fold(0, |sum, line| {
            let LineData {
                id, is_possible, ..
            } = parse_line(line);
            if is_possible {
                sum + id
            } else {
                sum
            }
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("example-1.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
