use prse::parse;
use std::collections::HashMap;

fn parse(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let lr = lines.next().unwrap();
    lines.next();

    let code = lines
        .map(|line| {
            let (key, left, right): (String, String, String) = parse!(line, "{} = ({}, {})");
            (key, (left, right))
        })
        .collect();
    (lr.to_owned(), code)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut target = "AAA".to_owned();
    let mut count = 0;

    let (lr, code) = parse(input);
    let mut lr_gen = lr.chars().cycle();

    while target != "ZZZ" {
        let (left, right) = &code[&target];
        target = match lr_gen.next().unwrap() {
            'L' => left.clone(),
            'R' => right.clone(),
            _ => unreachable!(),
        };
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    let (lr, code) = parse(input);
    let mut lr_gen = lr.chars().cycle();

    Some(count)
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 8, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 8, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
