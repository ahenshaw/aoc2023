use std::collections::HashSet;

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let mut winning = HashSet::<u32>::new();

            let (left, right) = line.split_once("|").unwrap();
            let (_, left) = left.split_once(":").unwrap();
            for item in left.split_ascii_whitespace() {
                winning.insert(item.parse::<u32>().unwrap());
            }

            right
                .split_ascii_whitespace()
                .filter(|s| winning.contains(&s.parse::<u32>().unwrap()))
                .count()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .filter(|&count| *count > 0)
            .map(|&count| 2u32.pow(count as u32 - 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse(input);
    let mut to_check: Vec<_> = (0..cards.len()).collect();
    let mut num_cards = 0;
    while !to_check.is_empty() {
        let card = to_check.pop().unwrap();
        num_cards += 1;
        let num_winners = cards[card];
        to_check.extend(card + 1..card + 1 + num_winners);
    }

    Some(num_cards)
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 4, 1,
        ));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 4, 2,
        ));
        assert_eq!(result, Some(30));
    }
}
