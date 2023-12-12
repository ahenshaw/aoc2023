use prse::parse;
use regex::Regex;

fn get_groups(s: &str) -> Vec<usize> {
    s.split('.')
        .map(|sub| sub.len())
        .filter(|&l| l > 0)
        .collect()
}

fn stream(s: &str, a: char, b: char) -> String {
    let pat = s.to_string();
    for x in 0..(1u32.shl::<usize>(n)) {}
    "".to_string()
}

pub fn part_one(input: &str) -> Option<u32> {
    for line in input.lines() {
        let (condition, group) = line.split_once(' ').unwrap();
        let groups: Vec<usize> = parse!(group, "{:,:}");
        let q = condition.chars().filter(|&c| c == '?').count();
        println!("{:?}", get_groups(&condition));
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
