use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use prse::parse;

type Groups = Vec<usize>;

type Cache = HashMap<usize, Regex>;

/// convert groups into regexes, but only the
/// first group needs to be made into a capture group
fn get_re<'a>(groups: &[usize], cache: &'a Cache) -> &'a Regex {
    cache.get(&groups.len()).unwrap()
}

fn make_re(groups: &[usize]) -> Regex {
    let re = groups
        .into_iter()
        .enumerate()
        .map(|(i, count)| {
            if i == 0 {
                format!(r"([\?#]{{{count}}})")
            } else {
                format!(r"[\?#]{{{count}}}")
            }
        })
        .join(r"[\.\?]+");
    Regex::new(&re).unwrap()
}

fn count_ways(condition: &str, groups: &[usize], cache: &Cache) -> usize {
    let condition = condition.trim_start_matches('.');
    // println!("    {condition} {groups:?}");
    let re = get_re(&groups, cache);
    // println!("    {}", make_re(&groups));
    let mut slice = condition;
    let mut ways = 0;
    let mut ends = HashSet::<usize>::new();
    let mut offset = 0;
    loop {
        if let Some(captures) = re.captures(slice) {
            let cap = captures.get(1).unwrap();
            if groups.len() > 1 {
                if !ends.contains(&(cap.end() + offset)) {
                    let (_toss, rest) = slice.split_at(cap.end() + 1);
                    // println!("    toss: {toss}   rest: {rest}\n");
                    ways += count_ways(rest, &groups[1..], cache);
                    ends.insert(cap.end() + offset);
                }
            } else {
                ways = 1
            }
        } else {
            break;
        }
        let (_, keep) = slice.split_at(1);
        offset += 1;
        slice = keep;
    }
    ways
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut total = 0;
    for line in input.lines() {
        let mut cache = Cache::new();
        let (condition, group) = line.split_once(' ').unwrap();
        let groups: Groups = parse!(group, "{:,:}").into();
        for i in 0..groups.len() {
            let re = make_re(&groups[i..]);
            cache.insert(groups.len() - i, re);
        }

        let ways = count_ways(condition, &groups, &cache);
        // println!("Final: {line} {ways}");
        total += ways;
    }
    // None
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let repeat = 5;
    let mut total = 0;
    for line in input.lines() {
        let mut cache = Cache::new();
        let (condition, group) = line.split_once(' ').unwrap();
        let groups: Groups = parse!(group, "{:,:}").into();

        let condition = vec![condition].repeat(repeat).join("?");
        let groups = groups.repeat(repeat);
        for i in 0..groups.len() {
            cache.insert(groups.len() - i, make_re(&groups[i..]));
        }
        let ways = count_ways(&condition, &groups, &cache);
        println!("Final: {line} {ways}");
        total += ways;
    }
    Some(total)
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

        assert_eq!(result, Some(525152));
    }
}
