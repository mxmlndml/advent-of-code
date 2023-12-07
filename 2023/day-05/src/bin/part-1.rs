// struct maps {
//     seed_to_soil: &str,
//     soil_to_fertilizer: &str,
//     fertilizer_to_water: &str,
//     water_to_light: &str,
//     light_to_temperature: &str,

// }

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
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
    let mut locations = seeds;

    input
        .split("\n\n")
        .skip(1)
        .enumerate()
        .for_each(|(step, map)| {
            // dbg!(map);
            // if step == 3 {
            //     dbg!(&locations);
            // }
            let mut tmp = locations.clone();

            map.lines().skip(1).for_each(|line| {
                let parsed = line
                    .split(" ")
                    .map(|string| string.parse::<u64>().expect("must have map values"))
                    .collect::<Vec<u64>>();

                let destination = parsed[0];
                let source = parsed[1];
                let length = parsed[2];

                locations
                    .iter()
                    .enumerate()
                    // .inspect(|f| {
                    //     if step == 3 {
                    //         dbg!(f);
                    //     }
                    // })
                    .for_each(|(i, location)| {
                        if (source..source + length).contains(location) {
                            if step == 2 {
                                dbg!(i, location);
                            }
                            let offset = location - source;
                            tmp[i] = destination + offset;

                            // if step == 3 {
                            //     dbg!(locations[i]);
                            // }
                        }
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

    const EXAMPLE_SOLUTION: &str = "35";

    #[test]
    fn example1_output() {
        let input = include_str!("example.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
