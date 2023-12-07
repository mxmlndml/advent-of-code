#[derive(Clone)]
struct Seed {
    start: u64,
    length: u64,
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> String {
    let seeds = input
        .lines()
        .next()
        .expect("no line")
        .replace("seeds: ", "")
        .split(" ")
        .map(|seed_number| {
            seed_number
                .parse::<u64>()
                .expect("seed number must be a digit")
        })
        .collect::<Vec<u64>>();

    let mut tmp = vec![];
    let seeds = seeds[..]
        .chunks(2)
        .map(|chunk| Seed {
            start: chunk[0],
            length: chunk[1],
        })
        .collect::<Vec<Seed>>();

    let mut locations = seeds;

    input
        .split("\n\n")
        .skip(1)
        .enumerate()
        .for_each(|(step, map)| {
            let mut tmp = locations.clone();

            map.lines().skip(1).for_each(|line| {
                let parsed = line
                    .split(" ")
                    .map(|string| string.parse::<u64>().expect("must have map values"))
                    .collect::<Vec<u64>>();

                let destination = parsed[0];
                let source = parsed[1];
                let length = parsed[2];

                locations.iter().enumerate().for_each(|(i, location)| {
                    let range = source..source + length;

                    if range.contains(&location.start)
                        && range.contains(&(location.start + location.length))
                    {
                        // fully contained -> change start
                        let offset = location.start - source;
                        tmp[i].start = destination + offset;
                    } else if range.contains(&location.start) {
                        // only first bit contained -> change start of first bit
                        let offset = location.start - source;
                        tmp[i].start = destination + offset;
                    } else if range.contains(&(location.start + location.length)) {
                        // only last bit contained -> change length of first bit
                    }

                    // partially contained -> split and change start and length

                    // not contained -> skip

                    // if (source..source + length).contains(location) {
                    //     if step == 2 {
                    //         dbg!(i, location);
                    //     }
                    //     let offset = location - source;
                    //     tmp[i] = destination + offset;
                    // }
                });
            });

            locations = tmp;

            dbg!(step, &locations);
        });

    locations
        .iter()
        .min()
        .expect("must have locations")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION: &str = "46";

    #[test]
    fn example2_output() {
        let input = include_str!("example.txt");
        assert_eq!(part2(input), EXAMPLE_SOLUTION.to_string());
    }
}
