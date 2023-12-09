#[derive(Debug)]
struct Hand {
    hand_score: u16,
    order_score: u32,
    bid: u32,
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> String {
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
                        'T' => 9_usize,
                        'J' => 0_usize,
                        number => (number.to_digit(10).expect("no number card") - 1) as usize,
                    };

                    order_score += 14_u32.pow(5 - position as u32) * (value as u32 + 1);
                    hand[value] += 1;
                });

            let biggest_amount = hand[1..]
                .iter()
                .enumerate()
                .fold(1_usize, |acc, (i, amount)| {
                    if amount > &hand[acc as usize] {
                        i + 1
                    } else {
                        acc
                    }
                });

            hand[biggest_amount] += hand[0];
            let hand_score = hand
                .iter()
                .skip(1)
                .fold(0_u16, |acc, curr| acc + (*curr as u16).pow(2));

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

    const EXAMPLE_SOLUTION: &str = "5905";

    #[test]
    fn example2() {
        let input = include_str!("example.txt");
        assert_eq!(part2(input), EXAMPLE_SOLUTION.to_string());
    }
}
