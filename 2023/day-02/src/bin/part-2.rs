#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

const EXAMPLE_SOLUTION: &str = "2286";

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

fn parse_line(line: &str) -> Cubes {
    let mut minimum_cubes = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };

    if let [_, subsets] = line.split(": ").collect::<Vec<&str>>()[..] {
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

            if cubes.red > minimum_cubes.red {
                minimum_cubes.red = cubes.red
            }
            if cubes.green > minimum_cubes.green {
                minimum_cubes.green = cubes.green
            }
            if cubes.blue > minimum_cubes.blue {
                minimum_cubes.blue = cubes.blue
            }
        });
    }

    minimum_cubes
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    lines
        .iter()
        .fold(0, |sum, line| {
            let minimum_cubes = parse_line(line);
            sum + minimum_cubes.red * minimum_cubes.green * minimum_cubes.blue
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        assert_eq!(part2(input), EXAMPLE_SOLUTION.to_string());
    }
}
