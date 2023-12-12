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

/// In part two, the number of iterations is prohibitive,
/// so you have to realize that each target gets returned
/// to after n cycles.  Therefore, we're just looking for
/// n for each target and then finding the LCM of all of
/// those cycles.
///
pub fn part_two(input: &str) -> Option<usize> {
    let mut count: u32 = 0;

    let (lr, code) = parse(input);
    let mut lr_gen = lr.chars().cycle();
    let mut targets: Vec<String> = code.keys().filter(|k| k.ends_with('A')).cloned().collect();
    let mut cycles: Vec<usize> = vec![];

    while cycles.len() != targets.len() {
        let is_left = lr_gen.next().unwrap() == 'L';

        if targets.iter().any(|target| target.ends_with('Z')) {
            cycles.push(count as usize);
        }
        targets = targets
            .iter()
            .map(|t| {
                if is_left {
                    code[t].0.clone()
                } else {
                    code[t].1.clone()
                }
            })
            .collect();
        count += 1;
    }
    let answer = cycles.into_iter().reduce(lcm).unwrap();
    Some(answer)
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
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
