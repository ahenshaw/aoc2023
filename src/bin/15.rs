pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
        .replace('\n', "")
        .split(',')
        .map(|step| {
            step.bytes()
                .fold(0, |acc: u64, ch| ((acc + ch as u64) * 17) % 256)
        })
        .sum();
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(15);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 15, 1,
        ));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 15, 2,
        ));
        assert_eq!(result, None);
    }
}
