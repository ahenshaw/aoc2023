use grid::*;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Beam {
    row: isize,
    col: isize,
    dir: Direction,
}

type Machine = Grid<char>;
type Visited = HashSet<Beam>;

impl Beam {
    fn new(row: isize, col: isize, dir: Direction) -> Self {
        Self { row, col, dir }
    }

    fn step(&self, machine: &Machine) -> Vec<Beam> {
        use Direction::*;
        let mut beams: Vec<Beam> = vec![];

        let (dr, dc) = match self.dir {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        };
        let (row, col) = (self.row + dr, self.col + dc);
        let cell = machine[(row as usize, col as usize)];

        match (cell, self.dir) {
            ('#', _) => (),
            ('.', _) => beams.push(Beam::new(row, col, self.dir)),
            ('|', Up | Down) => beams.push(Beam::new(row, col, self.dir)),
            ('-', Left | Right) => beams.push(Beam::new(row, col, self.dir)),
            ('|', Left | Right) => {
                beams.extend(vec![Beam::new(row, col, Up), Beam::new(row, col, Down)])
            }
            ('-', Up | Down) => {
                beams.extend(vec![Beam::new(row, col, Left), Beam::new(row, col, Right)])
            }
            ('/', Right) => beams.push(Beam::new(row, col, Up)),
            ('/', Left) => beams.push(Beam::new(row, col, Down)),
            ('/', Up) => beams.push(Beam::new(row, col, Right)),
            ('/', Down) => beams.push(Beam::new(row, col, Left)),
            ('\\', Right) => beams.push(Beam::new(row, col, Down)),
            ('\\', Left) => beams.push(Beam::new(row, col, Up)),
            ('\\', Up) => beams.push(Beam::new(row, col, Left)),
            ('\\', Down) => beams.push(Beam::new(row, col, Right)),

            _ => unreachable!(),
        }

        beams
    }
}

fn prepare(input: &str) -> Machine {
    let mut machine: Machine = grid![];
    for line in input.lines() {
        machine.push_row(line.chars().collect_vec());
    }
    let mut padded = Machine::init(machine.rows() + 2, machine.cols() + 2, '#');
    machine
        .indexed_iter()
        .for_each(|((r, c), cell)| padded[(r + 1, c + 1)] = *cell);
    padded
}

fn run(machine: &Machine, row: usize, col: usize, dir: Direction) -> usize {
    let mut beams: Vec<Beam> = vec![Beam::new(row as isize, col as isize, dir)];
    let mut visited = Visited::new();
    while let Some(beam) = beams.pop() {
        if !visited.contains(&beam) {
            visited.insert(beam);
            let result = beam.step(&machine);
            beams.extend(result);
        }
    }
    let cell_only: HashSet<(isize, isize)> = visited.into_iter().map(|b| (b.row, b.col)).collect();
    cell_only.len() - 1
}

pub fn part_one(input: &str) -> Option<usize> {
    let machine = prepare(input);
    Some(run(&machine, 1, 0, Direction::Right))
}

pub fn part_two(input: &str) -> Option<usize> {
    use Direction::*;
    let machine = prepare(input);
    let (rows, cols) = machine.size();

    let a = (0..rows).map(|r| run(&machine, r, 0, Right)).max().unwrap();
    let b = (0..rows)
        .map(|r| run(&machine, r, cols - 1, Left))
        .max()
        .unwrap();
    let c = (0..cols).map(|c| run(&machine, 0, c, Down)).max().unwrap();
    let d = (0..cols)
        .map(|c| run(&machine, rows - 1, c, Up))
        .max()
        .unwrap();

    [a, b, c, d].iter().max().copied()
}

advent_of_code::main!(16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 16, 1,
        ));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 16, 2,
        ));
        assert_eq!(result, Some(51));
    }
}
