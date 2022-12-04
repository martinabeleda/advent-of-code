pub fn part_a(input: &str) -> usize {
    let mut sum: usize = 0;
    for mut split in input.lines().map(|l| l.splitn(4, |c| c == '-' || c == ',')) {
        let l1 = split.next().unwrap().parse::<usize>().unwrap();
        let l2 = split.next().unwrap().parse::<usize>().unwrap();
        let r1 = split.next().unwrap().parse::<usize>().unwrap();
        let r2 = split.next().unwrap().parse::<usize>().unwrap();

        if ((l1 <= r1) && (l2 >= r2)) || ((l1 >= r1) && (l2 <= r2)) {
            sum = sum + 1;
        }
    }
    sum
}

pub fn part_b(input: &str) -> usize {
    let mut sum: usize = 0;
    for mut split in input.lines().map(|l| l.splitn(4, |c| c == '-' || c == ',')) {
        let l1 = split.next().unwrap().parse::<usize>().unwrap();
        let l2 = split.next().unwrap().parse::<usize>().unwrap();
        let r1 = split.next().unwrap().parse::<usize>().unwrap();
        let r2 = split.next().unwrap().parse::<usize>().unwrap();

        if !(l2 < r1) && !(r2 < l1) {
            sum = sum + 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 2);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 498);
    }

    #[test]
    fn part_b_sample() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 4);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 859);
    }
}
