use grid::*;
use itertools::Itertools;

type Platform = Grid<char>;

fn roll_north(platform: &Platform) -> Platform {
    let mut rolled: Platform = grid![];
    for col in platform.iter_cols() {
        let mut rolled_col: Vec<char> = col.copied().collect();
        loop {
            let mut dirty = false;
            for (i, (&dst, &src)) in rolled_col.clone().iter().tuple_windows().enumerate() {
                if src == 'O' && dst == '.' {
                    rolled_col.swap(i, i + 1);
                    dirty = true;
                }
            }
            if !dirty {
                break;
            }
        }
        rolled.push_col(rolled_col);
    }
    rolled
}

fn parse(input: &str) -> Grid<char> {
    let mut platform: Grid<char> = grid![];
    for row in input.lines() {
        platform.push_row(row.chars().collect());
    }
    platform
}

pub fn part_one(input: &str) -> Option<usize> {
    let platform = parse(input);
    let mut load = 0;
    let rolled = roll_north(&platform);
    let rows = rolled.rows();
    for ((row, _), &c) in rolled.indexed_iter() {
        if c == 'O' {
            load += rows - row;
        }
    }

    Some(load)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(14);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 14, 1,
        ));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 14, 2,
        ));
        assert_eq!(result, None);
    }
}
