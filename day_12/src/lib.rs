use std::collections::HashMap;

fn build_grid(input: &str) -> Vec<Vec<usize>> {
    let height_map: HashMap<char, usize> = ('a'..='z').zip(0..26).collect();

    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let v: Vec<_> = line
            .chars()
            .enumerate()
            .map(|(j, c)| {
                if c == 'S' {
                    start = (i, j);
                    0
                } else if c == 'E' {
                    end = (i, j);
                    0
                } else {
                   height_map[&c] 
                }
            })
            .collect();
        grid.push(v);
    }
    println!("{:?}", grid);
    grid
}


pub fn part_a(input: &str) -> usize {
    let grid = build_grid(&input);
    0
}

pub fn part_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 0);
    }

    // #[test]
    // fn part_a() {
    //     assert_eq!(super::part_a(include_str!("input.txt")), 0);
    // }
    //
    // #[test]
    // fn part_b_sample() {
    //     assert_eq!(super::part_b(include_str!("sample.txt")), 0);
    // }
    //
    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(include_str!("input.txt")), 0);
    // }
}
