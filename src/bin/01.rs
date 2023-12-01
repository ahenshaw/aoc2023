#![allow(unused_variables)]

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
    let mut total: u32 = 0;
    for line in input.lines() {
        let line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let tens = digits.first()?.to_digit(10)?;
        let ones = digits.last()?.to_digit(10)?;
        total += 10 * tens + ones;
    }
    Some(total)
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
