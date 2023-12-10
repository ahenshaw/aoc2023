#![allow(unused_variables)]
use std::ops::Range;

use prse::parse;

type Seeds = Vec<u64>;
type Maps = Vec<Map>;
type Mappings = Vec<Maps>;

#[derive(Debug)]
pub struct Map {
    pub dst_start: u64,
    pub src_start: u64,
    pub length: u64,
    pub range: Range<u64>,
}

impl Map {
    fn get(&self, src: &u64) -> Option<u64> {
        if !(self.src_start..self.src_start + self.length).contains(&src) {
            return None;
        }
        Some(src - self.src_start + self.dst_start)
    }
}
fn get(maps: &Maps, src: &u64) -> u64 {
    let dst = maps
        .iter()
        .map(|map| map.get(src))
        .find(|&value| value.is_some());

    match dst {
        None => *src,
        Some(value) => value.unwrap(),
    }
}

fn puzzle(input: &str) -> (Seeds, Mappings) {
    let seeds: Seeds;
    let mut sections = input.split("\n\n");

    parse!(sections.next().unwrap(), "seeds: {seeds: :}");
    let mappings: Mappings = sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let (dst_start, src_start, length): (u64, u64, u64) = parse!(line, "{} {} {}");
                    Map {
                        dst_start,
                        src_start,
                        length,
                        range: src_start..src_start + length,
                    }
                })
                .collect::<Maps>()
        })
        .collect();
    (seeds, mappings)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, mappings) = puzzle(input);
    let result = seeds
        .iter()
        .map(|seed| {
            let mut src = *seed;
            for m in &mappings {
                src = get(m, &src);
            }
            src
        })
        .min();
    match result {
        Some(value) => Some(value as u32),
        None => None,
    }
}

fn get_split(maps: &Maps, r: Range<u64>) -> Vec<Range<u64>> {
    let mut result: Vec<Range<u64>> = vec![];
    for map in maps {
        let begin = map.range.contains(&r.start);
        let end = map.range.contains(&r.end);
        dbg!(&r);
        let range_len = r.end - r.start;
        match (begin, end) {
            // nothing in the mapping range
            (false, false) => (),
            // everything in the mapping range
            (true, true) => {
                let dst_start = map.dst_start + r.start - map.src_start;
                let dst_end = map.dst_start + range_len;
                result.push(dst_start..dst_end);
            }
            // src begin in range, but not end
            (true, false) => {
                let extra = r.end - map.range.end;
                let dst_end = map.dst_start + map.length;
                let dst_start = dst_end - range_len + extra;
                result.push(dst_start..dst_end);
            }
            // src end in range, but not start
            (false, true) => {
                let extra = map.src_start - r.start;
                let dst_end = map.dst_start + range_len - extra;
                let dst_start = map.dst_start;
                result.push(dst_start..dst_end);
            }
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, mappings) = puzzle(input);
    let seeds: Vec<_> = seeds.chunks(2).map(|w| w[0]..w[0] + w[1]).collect();

    let mut ranges = seeds.clone();
    for maps in mappings {
        ranges = ranges
            .into_iter()
            .flat_map(|range| get_split(&maps, range).into_iter())
            .collect();
    }
    Some(ranges.into_iter().map(|r| r.start).min().unwrap())
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 5, 1,
        ));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 5, 2,
        ));
        assert_eq!(result, Some(46));
    }
}
