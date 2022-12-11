use std::collections::HashSet;

fn move_h(h_pos: &mut (i32, i32), dir: &str) {
    match dir {
        "R" => {
            h_pos.1 += 1;
        }
        "U" => {
            h_pos.0 += 1;
        }
        "D" => {
            h_pos.0 -= 1;
        }
        "L" => {
            h_pos.1 -= 1;
        }
        _ => panic!("Invalid move {}", dir),
    }
}

fn move_t(
    mut t_moves: Option<&mut HashSet<(i32, i32)>>,
    h_pos: (i32, i32),
    t_pos: &mut (i32, i32),
    dir: &str,
) {
    match dir {
        "R" => {
            let (y_dist, x_dist) = dist(h_pos, *t_pos);
            if x_dist == 2 && y_dist == 0 {
                t_pos.1 += 1;
            } else if x_dist == 2 && y_dist == 1 {
                t_pos.1 += 1;
                t_pos.0 += if h_pos.0 - t_pos.0 > 0 { 1 } else { -1 }
            }
        }
        "U" => {
            let (y_dist, x_dist) = dist(h_pos, *t_pos);
            if y_dist == 2 && x_dist == 0 {
                t_pos.0 += 1;
            } else if y_dist == 2 && x_dist == 1 {
                t_pos.0 += 1;
                t_pos.1 += if h_pos.1 - t_pos.1 > 0 { 1 } else { -1 }
            }
        }
        "D" => {
            let (y_dist, x_dist) = dist(h_pos, *t_pos);
            if y_dist == 2 && x_dist == 0 {
                t_pos.0 -= 1;
            } else if y_dist == 2 && x_dist == 1 {
                t_pos.0 -= 1;
                t_pos.1 += if h_pos.1 - t_pos.1 > 0 { 1 } else { -1 }
            }
        }
        "L" => {
            let (y_dist, x_dist) = dist(h_pos, *t_pos);
            if x_dist == 2 && y_dist == 0 {
                t_pos.1 -= 1;
            } else if x_dist == 2 && y_dist == 1 {
                t_pos.1 -= 1;
                t_pos.0 += if h_pos.0 - t_pos.0 > 0 { 1 } else { -1 }
            }
        }
        _ => panic!("Invalid move {}", dir),
    }
    match t_moves {
        Some(ref mut h) => {
            h.insert(*t_pos);
        }
        None => (),
    };
}

fn dist(h_pos: (i32, i32), t_pos: (i32, i32)) -> (i32, i32) {
    ((h_pos.0 - t_pos.0).abs(), (h_pos.1 - t_pos.1).abs())
}

pub fn part_a(input: &str) -> usize {
    let mut t_pos = (0, 0);
    let mut h_pos = (0, 0);
    let mut t_moves: HashSet<(i32, i32)> = HashSet::new();
    t_moves.insert(t_pos);

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse::<usize>().unwrap();
        for _ in 0..steps {
            move_h(&mut h_pos, direction);
            move_t(Some(&mut t_moves), h_pos, &mut t_pos, direction);
        }
    }
    t_moves.len()
}

fn move_t_b(
    mut t_moves: Option<&mut HashSet<(i32, i32)>>,
    h_pos: (i32, i32),
    t_pos: &mut (i32, i32),
    dir: &str,
) {
    if !((h_pos.0 - t_pos.0).abs() >= 1 && (h_pos.1 - t_pos.1).abs() >= 1) {
        if h_pos.1 > t_pos.1 {
            t_pos.1 += 1;
        } else if h_pos.1 < t_pos.1 {
            t_pos.1 -= 1;
        }
        if h_pos.0 > t_pos.0 {
            t_pos.0 += 1
        } else if h_pos.0 < t_pos.0 {
            t_pos.0 -= 1
        }
    }
    match t_moves {
        Some(ref mut h) => {
            h.insert(*t_pos);
        }
        None => (),
    };
}

pub fn part_b(input: &str) -> usize {
    let mut h_pos = (0, 0);
    let mut t_poses = vec![(0, 0); 9];

    let mut t_moves: HashSet<(i32, i32)> = HashSet::new();
    t_moves.insert(t_poses[t_poses.len() - 1]);

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse::<usize>().unwrap();
        for _ in 0..steps {
            move_h(&mut h_pos, direction);
            move_t_b(Some(&mut t_moves), h_pos, &mut t_poses[0], direction);

            // println!("line: {}, h_pos: {:?}, t_poses: {:?}", line, h_pos, t_poses);
            // Move each knot
            for i in 1..t_poses.len() {
                let tmp = if i == t_poses.len() - 1 {
                    Some(&mut t_moves)
                } else {
                    None
                };
                move_t_b(tmp, t_poses[i - 1], &mut t_poses[i], direction);
            }
            println!("i={} h_pos={:?}, t_poses={:?}", line, h_pos, t_poses);
        }
    }
    t_moves.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 13);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 6332);
    }

    #[test]
    fn part_b_sample() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 36);
    }

    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(include_str!("input.txt")), 0);
    // }
}
