use regex::Regex;

type SymbolInfo = Vec<(String, i32, i32)>;
type NumberInfo = Vec<(u32, i32, i32, i32)>;

fn parse(input: &str) -> (SymbolInfo, NumberInfo) {
    let re = Regex::new(r"\d+|[^\.\d]+").unwrap();
    let mut symbols: SymbolInfo = vec![];
    let mut numbers: NumberInfo = vec![];

    for (row, line) in input.lines().enumerate() {
        for match_case in re.find_iter(line) {
            let item = match_case.as_str();
            let start = match_case.start() as i32;
            let end = match_case.end() as i32;
            match item.parse::<u32>() {
                Ok(num) => numbers.push((num, row as i32, start, end)),
                Err(_) => symbols.push((item.to_owned(), row as i32, start)),
            }
        }
    }
    (symbols, numbers)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (symbols, numbers) = parse(input);
    let mut total: u32 = 0;
    for (num, row, start, end) in &numbers {
        if symbols
            .iter()
            .any(|(_, srow, scol)| (srow - row).abs() <= 1 && (start - 1) <= *scol && end >= scol)
        {
            total += num;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (symbols, numbers) = parse(input);
    let mut total: u32 = 0;
    for (symbol, srow, scol) in &symbols {
        if symbol == "*" {
            let mut gears: Vec<u32> = Vec::new();

            for (num, row, start, end) in &numbers {
                if (srow - row).abs() <= 1 && (start - 1) <= *scol && end >= scol {
                    gears.push(*num);
                }
            }
            if gears.len() == 2 {
                total += gears.pop().unwrap() * gears.pop().unwrap();
            }
        }
    }
    Some(total)
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 3, 1,
        ));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 3, 2,
        ));
        assert_eq!(result, Some(467835));
    }
}
