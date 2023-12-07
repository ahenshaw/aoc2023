// use scan_fmt::scan_fmt;
use prse::parse;

pub fn part_one(input: &str) -> Option<u32> {
    let red = 12;
    let green = 13;
    let blue = 14;

    let mut total = 0;
    for line in input.lines() {
        let (game, draws) = line.split_once(": ").unwrap();
        let gid: u32 = parse!(game, "Game {}");

        let mut flag = true;
        for draw in draws.split("; ") {
            for cube in draw.split(", ") {
                let (count, color): (u32, String) = parse!(cube, "{} {}");
                if match color.as_str() {
                    "red" => red,
                    "green" => green,
                    "blue" => blue,
                    _ => unreachable!(),
                } < count
                {
                    flag = false
                }
            }
        }
        if flag {
            total += gid;
        };
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let (game, draws) = line.split_once(": ").unwrap();
        let _gid: u32 = parse!(game, "Game {}");

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for draw in draws.split("; ") {
            for cube in draw.split(", ") {
                // let (count, color) = sscanf!(cube, "{} {}", u32, String).unwrap();
                let (count, color): (u32, String) = parse!(cube, "{} {}");
                match color.as_str() {
                    "red" => red = red.max(count),
                    "green" => green = green.max(count),
                    "blue" => blue = blue.max(count),
                    _ => unreachable!(),
                }
            }
        }
        let power = red * green * blue;
        total += power;
    }
    Some(total)
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 2, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 2, 2,
        ));
        assert_eq!(result, Some(2286));
    }
}
