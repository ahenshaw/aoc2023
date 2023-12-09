use prse::parse;

type Sequence = Vec<i32>;

pub fn part_one(input: &str) -> Option<i32> {
    let mut total = 0;
    for line in input.lines() {
        let mut ends: Vec<i32> = vec![];
        let mut sequence: Sequence = parse!(line, "{: :}");
        loop {
            if sequence.iter().all(|&x| x == 0) {
                break;
            }
            ends.push(*sequence.last().unwrap());
            sequence = sequence.windows(2).map(|w| w[1] - w[0]).collect();
        }
        total += ends.iter().sum::<i32>();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut total = 0;
    for line in input.lines() {
        let mut starts: Vec<i32> = vec![];
        let mut sequence: Sequence = parse!(line, "{: :}");
        loop {
            if sequence.iter().all(|&x| x == 0) {
                break;
            }
            starts.push(*sequence.first().unwrap());
            sequence = sequence.windows(2).map(|w| w[1] - w[0]).collect();
        }
        starts.reverse();
        total += starts.into_iter().reduce(|acc, x| (x - acc)).unwrap();
    }
    Some(total)
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 9, 1,
        ));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 9, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
