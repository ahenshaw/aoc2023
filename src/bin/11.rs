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

    let mut ng = Grid::new((0, 0));
    for (loc, &value) in space.occupied_entries() {
        let need_rows = empty_rows.iter().filter(|&r| loc.row.0 > **r).count() as isize;
        let need_cols = empty_cols.iter().filter(|&c| loc.column.0 > **c).count() as isize;
        let new_loc = *loc + Vector::new(need_rows * extras, need_cols * extras);
        ng.insert(new_loc, value);
    }
    ng
}

fn parse(input: &str) -> Grid {
    let mut space = Grid::new((0, 0));
    let mut id = 1;
    for (row, line) in input.lines().enumerate() {
        for (column, _) in line.match_indices('#') {
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
    let space = expand(space, 99);

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
