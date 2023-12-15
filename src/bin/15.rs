use std::collections::HashMap;

fn hash(step: &str) -> usize {
    step.bytes()
        .fold(0, |acc: usize, ch| ((acc + ch as usize) * 17) % 256)
}
pub fn part_one(input: &str) -> Option<usize> {
    let result: usize = input.replace('\n', "").split(',').map(hash).sum();
    Some(result)
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focus: Option<usize>,
}

#[derive(Debug)]
struct Box {
    id: usize,
    lenses: Vec<Lens>,
    lu: HashMap<String, usize>,
}

type Boxes = Vec<Box>;

#[derive(Debug)]
enum Op {
    Remove(Lens),
    Add(Lens),
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: Boxes = (0..256).map(Box::new).collect();

    for op in parse(input) {
        match op {
            Op::Remove(lens) => boxes[lens.hash()].remove(lens),
            Op::Add(lens) => boxes[lens.hash()].add(lens),
        }
    }

    Some(score(&boxes))
}

fn score(boxes: &Boxes) -> usize {
    boxes
        .iter()
        .map(|b| {
            b.lenses
                .iter()
                .filter(|lens| lens.focus.is_some())
                .enumerate()
                .map(|(i, lens)| (b.id + 1) * (i + 1) * lens.focus.unwrap())
                .sum::<usize>()
        })
        .sum()
}

fn parse(input: &str) -> Vec<Op> {
    input
        .replace('\n', "")
        .split(',')
        .map(|s| {
            if s.contains('-') {
                let label = s.replace('-', "");
                let focus = None;
                Op::Remove(Lens { label, focus })
            } else {
                let (label, focal_length) = s.split_once('=').unwrap();
                let label = label.to_string();
                let focus = Some(focal_length.parse::<usize>().unwrap());
                Op::Add(Lens { label, focus })
            }
        })
        .collect()
}

impl Lens {
    fn hash(&self) -> usize {
        hash(&self.label)
    }
}

impl Box {
    fn new(id: usize) -> Self {
        Box {
            id,
            lenses: Vec::new(),
            lu: HashMap::new(),
        }
    }

    fn add(&mut self, lens: Lens) {
        let label = lens.label.clone();
        if self.lu.contains_key(&label) {
            self.lenses[*self.lu.get(&label).unwrap()] = lens;
        } else {
            self.lu.insert(lens.label.clone(), self.lenses.len());
            self.lenses.push(lens);
        }
    }

    fn remove(&mut self, lens: Lens) {
        if let Some(index) = self.lu.remove(&lens.label) {
            // just mark the old entry as invalid and don't move anything
            self.lenses[index].focus = None;
        }
    }
}

advent_of_code::main!(15);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_with_part(
            "examples", 15, 1,
        ));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_with_part(
            "examples", 15, 2,
        ));
        assert_eq!(result, Some(145));
    }
}
