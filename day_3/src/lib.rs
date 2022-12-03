use itertools::Itertools;
use std::collections::HashSet;

fn set_from_line(line: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for char in line.chars() {
        set.insert(char);
    }
    set
}

fn priority(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 96
    } else {
        c as usize - 38
    }
}

pub fn part_a(input: &str) -> usize {
    let mut total: usize = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let left_contents = set_from_line(left);
        let right_contents = set_from_line(right);
        let char = left_contents.intersection(&right_contents).next().unwrap();
        let val = priority(*char);
        total = total + val;
    }
    total
}

pub fn part_b(input: &str) -> usize {
    let mut total: usize = 0;
    for group in &input.lines().chunks(3) {
        let mut sets: Vec<HashSet<char>> = Vec::new();
        for line in group {
            sets.push(set_from_line(line))
        }

        let intersection = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });

        let val = priority(*intersection.iter().next().unwrap());
        total = total + val;
    }
    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 157);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 7875);
    }

    #[test]
    fn part_b_sample() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 70);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2479);
    }
}
