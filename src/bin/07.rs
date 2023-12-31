use counter::Counter;
use std::collections::HashSet;

type Hand = Vec<u32>;
type Hands = Vec<(Hand, u32)>;

#[derive(Hash, PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
pub enum Value {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn parse(input: &str, part: u32) -> Hands {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid: u32 = bid.parse().unwrap();
            let hand: Hand = hand
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => {
                        if part == 1 {
                            11
                        } else {
                            1
                        }
                    }
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                })
                .collect();
            (hand, bid)
        })
        .collect()
}

fn max_same_kind(hand: &Hand) -> usize {
    let counts = hand.iter().collect::<Counter<_>>();
    counts.k_most_common_ordered(1)[0].1
}

fn get_hand_value(hand: &Hand) -> Value {
    let ranks: HashSet<&u32> = hand.iter().collect();
    match ranks.len() {
        1 => Value::FiveOfAKind,
        2 => {
            if max_same_kind(hand) == 4 {
                Value::FourOfAKind
            } else {
                Value::FullHouse
            }
        }
        3 => {
            if max_same_kind(hand) == 3 {
                Value::ThreeOfAKind
            } else {
                Value::TwoPair
            }
        }
        4 => Value::OnePair,
        5 => Value::HighCard,
        _ => unreachable!(),
    }
}

fn get_wildcard_hand_value(hand: &Hand) -> Value {
    (2..=14)
        .map(|joker| {
            let new_hand: Hand = hand
                .iter()
                .map(|c| if *c == 1 { joker } else { *c })
                .collect();
            get_hand_value(&new_hand)
        })
        .max()
        .unwrap()
}

fn get_value(mut ordering: Vec<(u32, &u32)>) -> u32 {
    ordering.sort();
    ordering
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * **bid)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = parse(input, 1);
    let ordering: Vec<(u32, &u32)> = hands
        .iter()
        .map(|(hand, bid)| {
            let mut slider: u32 = 64_000_000;
            let mut value = get_hand_value(hand) as u32 * slider;
            for card in hand {
                slider /= 20;
                value += card * slider;
            }
            (value, bid)
        })
        .collect();
    Some(get_value(ordering))
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = parse(input, 2);
    let ordering: Vec<(u32, &u32)> = hands
        .iter()
        .map(|(hand, bid)| {
            let mut slider: u32 = 64_000_000;
            let mut value = get_wildcard_hand_value(hand) as u32 * slider;
            for card in hand {
                slider /= 20;
                value += card * slider;
            }
            (value, bid)
        })
        .collect();
    Some(get_value(ordering))
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 7, 1,
        ));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 7, 2,
        ));
        assert_eq!(result, Some(5905));
    }
}
