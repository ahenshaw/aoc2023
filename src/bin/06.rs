use prse::parse;

fn puzzle(input: &str) -> Vec<(u32, u32)> {
    let mut lines = input.lines();
    let (_, numbers) = lines.next().unwrap().split_once(':').unwrap();
    let time: Vec<u32> = numbers
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let (_, numbers) = lines.next().unwrap().split_once(':').unwrap();
    let distance: Vec<u32> = numbers
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    time.into_iter().zip(distance).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        puzzle(input)
            .iter()
            .map(|(time, distance)| {
                (0..*time)
                    .map(|t| t * (time - t))
                    .filter(|d| d > distance)
                    .count() as u32
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.replace(' ', "");
    let mut lines = input.lines();

    let time: u64 = parse!(lines.next().unwrap(), "Time:{}");
    let distance: u64 = parse!(lines.next().unwrap(), "Distance:{}");
    Some(
        (0..time)
            .map(|t| t * (time - t))
            .filter(|d| d > &distance)
            .count() as u32,
    )
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 6, 1,
        ));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 6, 2,
        ));
        assert_eq!(result, Some(71503));
    }
}
