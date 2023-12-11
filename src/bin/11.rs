use gridly::{prelude::*, shorthand::L};
use gridly_grids::SparseGrid;
use itertools::Itertools;
use std::collections::HashSet;

type Grid = SparseGrid<usize>;
fn expand(space: Grid, extras: isize) -> Grid {
    let mut rows = HashSet::<isize>::new();
    let mut cols = HashSet::<isize>::new();
    for (loc, _) in space.occupied_entries() {
        rows.insert(loc.row.0);
        cols.insert(loc.column.0);
    }
    let size = space.dimensions();
    let all_rows = HashSet::from_iter(0..size.rows.0);
    let all_cols = HashSet::from_iter(0..size.columns.0);
    let empty_rows: HashSet<&isize> = all_rows.difference(&rows).collect();
    let empty_cols: HashSet<&isize> = all_cols.difference(&cols).collect();

    // reverse sort the empties so that they don't screw up the checking
    let empty_rows = empty_rows
        .into_iter()
        .sorted()
        .rev()
        .collect::<Vec<&isize>>();
    let empty_cols = empty_cols
        .into_iter()
        .sorted()
        .rev()
        .collect::<Vec<&isize>>();

    let mut ng = Grid::new((0, 0));
    for (loc, &value) in space.occupied_entries() {
        let mut loc = loc.clone();
        for empty_row in &empty_rows {
            if loc.row.0 > **empty_row {
                loc.row += Rows(extras);
            }
        }
        for empty_col in &empty_cols {
            if loc.column.0 > **empty_col {
                loc.column += Columns(extras);
            }
        }
        ng.insert(loc, value);
    }
    ng
}

fn parse(input: &str) -> Grid {
    let mut space = Grid::new((0, 0));
    let mut id = 1;
    for (row, line) in input.lines().enumerate() {
        for column in line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i)
        {
            space.insert(L(row as isize, column as isize), id);
            id += 1;
        }
    }
    space
}

pub fn part_one(input: &str) -> Option<isize> {
    let space = parse(input);
    let space = expand(space, 1);

    let sum_distance = space
        .occupied_entries()
        .combinations(2)
        .map(|pair| {
            let a = pair[0].0.as_location();
            let b = pair[1].0.as_location();

            let delta = a - b;
            delta.rows.0.abs() + delta.columns.0.abs()
        })
        .sum();
    Some(sum_distance)
}

pub fn part_two(input: &str) -> Option<isize> {
    let space = parse(input);
    let space = expand(space, 999_999);

    let sum_distance = space
        .occupied_entries()
        .combinations(2)
        .map(|pair| {
            let a = pair[0].0.as_location();
            let b = pair[1].0.as_location();

            let delta = a - b;
            delta.rows.0.abs() + delta.columns.0.abs()
        })
        .sum();
    Some(sum_distance)
}

advent_of_code::main!(11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 11, 1,
        ));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 11, 2,
        ));
        assert_eq!(result, Some(8410));
    }
}
