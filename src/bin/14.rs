use grid::*;
use itertools::Itertools;

type Platform = Grid<char>;

fn tilt(platform: &Platform) -> Platform {
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

fn calc_load(platform: &Platform) -> usize {
    let num_rows = platform.rows();
    platform
        .indexed_iter()
        .filter(|(_, &c)| c == 'O')
        .map(|((row, _), _)| num_rows - row)
        .sum()
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
    let rolled = tilt(&platform);

    Some(calc_load(&rolled))
}

struct CycleDetector(Vec<Platform>);
impl CycleDetector {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, item: &Platform) -> Option<usize> {
        if let Some(pos) = self.0.iter().position(|x| x == item) {
            return Some(self.0.len() + 1 - pos);
        } else {
            self.0.push(item.clone());
            None
        }
    }
    fn get(&mut self, index: usize) -> Platform {
        self.0.get(index).unwrap().clone()
    }
}
/// printed 300 scores, saw the pattern
/// and manually calculated the 1 billionth score

const CYCLES: usize = 20; //1000000000;
pub fn part_two(input: &str) -> Option<usize> {
    let mut platform = parse(input);
    let mut cycle_detector = CycleDetector::new();
    for cycle in 0..CYCLES {
        for _ in 0..4 {
            platform = tilt(&platform);
            platform.rotate_right();
        }
        println!("{cycle} {}", calc_load(&platform));
        // if let Some(cycle_len) = cycle_detector.add(&platform) {
        //     let base = cycle - cycle_len;
        //     println!("Cycles at {cycle_len}, base {base}");
        //     let index = ((CYCLES - base) % cycle_len) + base - 1;
        //     dbg!(&index);
        //     let platform = cycle_detector.get(index);
        //     if cycle == 20 {
        //         return Some(calc_load(&platform));
        //     };
        // }
    }
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
        assert_eq!(result, Some(64));
    }
}
