fn eval_pattern(pattern: &str) -> usize {
    // look for horizontal mirror line
    let rows: Vec<&str> = pattern.lines().collect();
    println!("{pattern}");
    let mut rrows = rows.clone();
    rrows.reverse();
    let num_rows = rows.len();
    let num_cols = rows[0].len();
    for i in 0..num_rows - 1 {
        if rrows[0..num_rows - i] == rows[i..] {
            return dbg!(100 * ((num_rows - i) / 2));
        }
        if rrows[i..] == rows[0..num_rows - i] {
            return dbg!(100 * ((num_rows + i) / 2));
        }
    }

    // create a transposed version of the pattern
    let mut cols: Vec<String> = (0..num_cols).map(|_| String::new()).collect();
    for row in &rows {
        for (i, c) in row.chars().enumerate() {
            cols[i].push(c);
        }
    }

    // look for vertical mirror line
    let mut rcols = cols.clone();
    rcols.reverse();
    let num_rows = cols.len();
    for i in 0..num_rows - 1 {
        if rcols[0..num_rows - i] == cols[i..] {
            return dbg!((num_rows - i) / 2);
        }
        if rcols[i..] == cols[0..num_rows - i] {
            return dbg!((num_rows + i) / 2);
        }
    }

    dbg!(0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut total = 0;
    for pattern in input.split("\n\n") {
        let score = eval_pattern(pattern);
        total += score;
    }
    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(13);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 13, 1,
        ));
        assert_eq!(result, Some(709));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 13, 2,
        ));
        assert_eq!(result, None);
    }
}
