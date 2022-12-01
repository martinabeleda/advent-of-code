use std::collections::BinaryHeap;

pub fn part_a(content: &str) -> i64 {
    let mut max: i64 = 0;
    for elf in content.trim().split("\n\n") {
        let calories: i64 = elf
            .trim()
            .split("\n")
            .map(|l| l.parse::<i64>().unwrap())
            .sum();
        if calories > max {
            max = calories;
        }
    }
    max
}

pub fn part_b(content: &str) -> i64 {
    let mut heap = BinaryHeap::with_capacity(3);
    for elf in content.trim().split("\n\n") {
        let calories: i64 = elf
            .trim()
            .split("\n")
            .map(|l| l.parse::<i64>().unwrap())
            .sum();
        heap.push(calories);
    }
    let mut vec = heap.into_sorted_vec();
    vec.reverse();
    vec.truncate(3);
    vec.into_iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 66616);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 199172);
    }
}
