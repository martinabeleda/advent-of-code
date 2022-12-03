use std::collections::HashSet;

pub fn part_a(input: &str) -> usize {
    let mut total: usize = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let mut left_contents = HashSet::new();
        let mut right_contents = HashSet::new();
        for char in left.chars() {
            left_contents.insert(char);
        }
        for char in right.chars() {
            right_contents.insert(char);
        }
        let char = left_contents.intersection(&right_contents).next();
        let val = match char {
            Some(char) => {
                if char.is_lowercase() {
                    *char as usize - 96
                } else {
                    *char as usize - 38
                }
            }
            None => panic!("Could not parse"),
        };
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
}
