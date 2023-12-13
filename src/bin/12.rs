use itertools::Itertools;
use std::ops::{Shl, Shr};

use prse::parse;

fn get_groups(s: &str) -> Vec<usize> {
    s.split('.')
        .map(|sub| sub.len())
        .filter(|&l| l > 0)
        .collect()
}

fn make_re(groups: Vec<usize>) -> String {
    groups
        .into_iter()
        .map(|count| r"([\?#]{count})")
        .join(r"[\.\?]+")
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let (condition, group) = line.split_once(' ').unwrap();
        let groups: Vec<usize> = parse!(group, "{:,:}");

        let indices: Vec<usize> = condition.match_indices('?').map(|(i, _)| i).collect();
        let n = indices.len();
        for mut x in 0..(1u32.shl(n)) {
            let mut out: Vec<char> = condition.chars().collect();
            for i in 0..n {
                out[indices[i]] = if x & 1 == 0 { '.' } else { '#' };
                x = x.shr(1);
            }
            let s: String = out.into_iter().collect();
            if get_groups(&s) == groups {
                total += 1
            }
        }
    }
    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(12);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 12, 1,
        ));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 12, 2,
        ));
        assert_eq!(result, None);
    }
}
