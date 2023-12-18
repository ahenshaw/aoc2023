fn parse(input: &str) -> usize {
    input.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = parse(input);
    Some(puzzle)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(17);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 17, 1,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 17, 2,
        ));
        assert_eq!(result, None);
    }
}
