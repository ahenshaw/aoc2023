#![allow(unused_variables)]

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let tens = digits.first()?.to_digit(10)?;
        let ones = digits.last()?.to_digit(10)?;
        total += 10 * tens + ones
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pattern = [
        r"\d", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let re = Regex::new(&pattern.join("|")).unwrap();

    let result = input
        .lines()
        .map(|line| {
            let digits = re
                .find_iter(line)
                .map(|m| {
                    let x = m.as_str();
                    if x.len() == 1 {
                        // has to be a digit
                        x.parse::<u32>().unwrap()
                    } else {
                        // has to be a spelled-out number, so find its position in table
                        pattern.iter().position(|&r| r == x).unwrap() as u32
                    }
                })
                .collect::<Vec<u32>>();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum();
    Some(result)
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(281));
    }
}
