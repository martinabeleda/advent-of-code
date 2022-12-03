pub fn part_a(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x| {
            return x.lines().flat_map(str::parse::<usize>).sum::<usize>();
        })
        .max()
        .unwrap()
}

pub fn part_b(input: &str) -> usize {
    let mut vec = input
        .split("\n\n")
        .map(|x| {
            return x.lines().flat_map(str::parse::<usize>).sum::<usize>();
        })
        .collect::<Vec<usize>>();

    vec.sort_by(|a, b| b.cmp(a));
    vec.into_iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 24000);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 66616);
    }

    #[test]
    fn part_b_sample() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 45000);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 199172);
    }
}
