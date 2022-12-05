use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn overlaps(&self, other: &Self) -> bool {
        return self.end >= other.start && other.end >= self.start;
    }
}

impl FromStr for Range {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<_> = s.split("-").collect();
        Ok(Self {
            start: pair[0].parse().unwrap(),
            end: pair[1].parse().unwrap(),
        })
    }
}

pub fn part_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .flat_map(|s| s.parse::<Range>())
                .collect::<Vec<Range>>()
        })
        .filter(|ranges| {
            let [a, b] = &ranges[..] else { panic!("Could not parse range!") };
            a.contains(b) || b.contains(a)
        })
        .count()
}

pub fn part_b(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .flat_map(|s| s.parse::<Range>())
                .collect::<Vec<Range>>()
        })
        .filter(|ranges| {
            let [a, b] = &ranges[..] else { panic!("Could not parse range!") };
            a.overlaps(b)
        })
        .count()
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
