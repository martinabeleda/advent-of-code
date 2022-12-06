use core::hash::Hash;
use std::collections::HashSet;

fn has_unqique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn part_a(input: &str) -> usize {
    let window_size: usize = 4;
    let binding = input.chars().collect::<Vec<char>>();
    let win = binding
        .windows(window_size)
        .position(|x| has_unqique_elements(x))
        .unwrap();
    win + window_size
}

pub fn part_b(input: &str) -> usize {
    let window_size: usize = 14;
    let binding = input.chars().collect::<Vec<char>>();
    let win = binding
        .windows(window_size)
        .position(|x| has_unqique_elements(x))
        .unwrap();
    win + window_size
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 7);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 1361);
    }

    #[test]
    fn part_b_sample() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 19);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 3263);
    }
}
