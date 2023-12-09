#[derive(Debug)]
struct Hand {
    hand_score: u8,
    order_score: u32,
    bid: u32,
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut line = line.split(" ");

            let mut order_score = 0_u32;
            let mut hand = [0_u8; 13];
            line.next()
                .expect("no hand")
                .chars()
                .enumerate()
                .for_each(|(position, card)| {
                    if position == 5 {
                        panic!("too many cards for a hand");
                    }

                    let value = match card {
                        'A' => 12_usize,
                        'K' => 11_usize,
                        'Q' => 10_usize,
                        'J' => 9_usize,
                        'T' => 8_usize,
                        number => (number.to_digit(10).expect("no number card") - 2) as usize,
                    };

                    order_score += 14_u32.pow(5 - position as u32) * value as u32;
                    hand[value] += 1;
                });

            let hand_score = hand.iter().fold(0_u8, |acc, curr| acc + curr.pow(2));

            let bid = line
                .next()
                .expect("no bid")
                .parse::<u32>()
                .expect("bid is not a number");

            Hand {
                hand_score,
                order_score,
                bid,
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort_unstable_by(|a, b| {
        if a.hand_score == b.hand_score {
            a.order_score.cmp(&b.order_score)
        } else {
            a.hand_score.cmp(&b.hand_score)
        }
    });

    hands
        .iter()
        .enumerate()
        .fold(0_u32, |acc, (pos, curr)| acc + (pos as u32 + 1) * curr.bid)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION: &str = "6440";

    #[test]
    fn example1() {
        let input = include_str!("example.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
