use gridly::{prelude::*, shorthand::L};
use gridly_grids::SparseGrid;

type Grid = SparseGrid<char>;

fn get_perimeter(input: &str) -> Vec<Location> {
    let mut grid: Grid = Grid::new((0, 0));
    let mut start = L(0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as isize;
            let col = col as isize;
            grid.insert((row, col), c);
            if c == 'S' {
                start = L(row, col);
            }
        }
    }

    // find a cell that is connected to the start cell
    let mut cursor = L(0, 0);
    for dir in [Up, Down, Right, Left] {
        let test = start + dir;
        let c = grid[test];
        match (dir, c) {
            (Up, '|' | '7' | 'F') => cursor = test,
            (Down, '|' | 'L' | 'J') => cursor = test,
            (Right, '-' | 'J' | '7') => cursor = test,
            (Left, '-' | 'F' | 'L') => cursor = test,
            _ => (),
        }
    }
    let mut perimeter = Vec::<Location>::new();
    perimeter.push(start);
    let mut prev = start;
    while grid[&cursor] != 'S' {
        perimeter.push(cursor);
        let dir = cursor - prev;
        let next_dir = match grid[&cursor] {
            '|' | '-' => dir,
            '7' | 'L' => dir.transpose(),
            'F' | 'J' => -dir.transpose(),
            _ => unreachable!(),
        };
        prev = cursor;
        cursor = cursor + next_dir;
    }
    perimeter
}

pub fn part_one(input: &str) -> Option<usize> {
    let perimeter = get_perimeter(input);
    Some((perimeter.len() + 1) / 2)
}
pub fn part_two(input: &str) -> Option<isize> {
    // add start point to end, to close the perimeter
    let mut perimeter = get_perimeter(input);
    let circumference = perimeter.len();

    perimeter.push(perimeter.first().unwrap().clone());
    let area: isize = perimeter
        .windows(2)
        .map(|w| w[0].row.0 * w[1].column.0 - w[1].row.0 * w[0].column.0)
        .sum::<isize>()
        .abs();
    Some((area - circumference as isize) / 2 + 1)
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 10, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 10, 2,
        ));
        assert_eq!(result, Some(8));
    }
}
