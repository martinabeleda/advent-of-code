fn visible_from_direction(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    let this = grid[i][j].to_digit(10).unwrap();
    let height = grid.len();
    let width = grid[0].len();

    let mut top = Vec::new();
    for x in 0..i {
        top.push(grid[x][j].to_digit(10).unwrap());
    }
    if this > *top.iter().max().unwrap() {
        return true;
    }

    let mut bottom = Vec::new();
    for x in i + 1..height {
        bottom.push(grid[x][j].to_digit(10).unwrap());
    }
    if this > *bottom.iter().max().unwrap() {
        return true;
    }

    let mut left = Vec::new();
    for y in 0..j {
        left.push(grid[i][y].to_digit(10).unwrap());
    }
    if this > *left.iter().max().unwrap() {
        return true;
    }

    let mut right = Vec::new();
    for y in j + 1..width {
        right.push(grid[i][y].to_digit(10).unwrap());
    }
    if this > *right.iter().max().unwrap() {
        return true;
    }
    return false;
}

fn count_boundary(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    (2 * height) + (2 * width) - 4
}

pub fn part_a(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut count = 0;
    for i in 1..grid.len() - 1 {
        let row = &grid[i];
        for j in 1..row.len() - 1 {
            let char = row[j];
            if visible_from_direction(i, j, &grid) {
                count += 1;
            }
        }
    }
    count + count_boundary(&grid)
}

// pub fn part_b(input: &str) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 21);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 0);
    }

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
