use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::string::ParseError;

const RADIX: u32 = 10;

#[derive(Debug)]
struct Move {
    crates: usize,
    source: u32,
    destination: u32,
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        Ok(Self {
            crates: vec[0] as usize,
            source: vec[1] as u32,
            destination: vec[2] as u32,
        })
    }
}

fn build_stacks(input: &str) -> HashMap<u32, VecDeque<char>> {
    let crates = input.lines().collect::<Vec<_>>();

    let indexes = crates
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(RADIX))
        .map(|(i, c)| (c.to_digit(RADIX).unwrap(), i))
        .collect::<HashMap<u32, usize>>();

    let mut stacks: HashMap<u32, VecDeque<char>> = HashMap::new();
    for (stack_id, index) in indexes {
        let mut stack: VecDeque<char> = VecDeque::new();
        for level in crates[0..crates.len() - 1].iter() {
            let c = level.chars().nth(index).unwrap();
            if c.is_ascii_uppercase() {
                stack.push_front(c);
            }
            stacks.insert(stack_id, stack.clone());
        }
    }
    stacks
}

fn build_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| l.parse::<Move>().unwrap())
        .collect::<Vec<Move>>()
}

fn result_from_stacks(stacks: &mut HashMap<u32, VecDeque<char>>) -> String {
    let mut res: Vec<char> = Vec::new();
    for i in 1..stacks.len() + 1 {
        let source: &mut VecDeque<char> = stacks.get_mut(&(i as u32)).unwrap();
        let val = source.pop_back().unwrap();
        res.push(val);
    }
    res.into_iter().collect()
}

pub fn part_a(input: &str) -> String {
    let mut split = input.split("\n\n");
    let mut stacks = build_stacks(split.next().unwrap());

    for step in build_moves(split.next().unwrap()) {
        for _ in 0..step.crates {
            let source: &mut VecDeque<char> = stacks.get_mut(&step.source).unwrap();
            let val = source.pop_back().unwrap();
            let dest: &mut VecDeque<char> = stacks.get_mut(&step.destination).unwrap();
            dest.push_back(val);
        }
    }
    result_from_stacks(&mut stacks)
}

pub fn part_b(input: &str) -> String {
    let mut split = input.split("\n\n");
    let mut stacks = build_stacks(split.next().unwrap());

    for step in build_moves(split.next().unwrap()) {
        let mut tmp: VecDeque<char> = VecDeque::new();
        let source: &mut VecDeque<char> = stacks.get_mut(&step.source).unwrap();
        for _ in 0..step.crates {
            let val = source.pop_back().unwrap();
            tmp.push_front(val);
        }

        let dest: &mut VecDeque<char> = stacks.get_mut(&step.destination).unwrap();
        for _ in 0..step.crates {
            let val = tmp.pop_front().unwrap();
            dest.push_back(val);
        }
    }
    result_from_stacks(&mut stacks)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), "CMZ");
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), "FCVRLMVQP");
    }

    #[test]
    fn part_b_sample() {
        assert_eq!(super::part_b(include_str!("sample.txt")), "MCD");
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), "RWLWGJGFD");
    }
}
