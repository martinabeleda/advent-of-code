use std::{collections::HashSet, str::FromStr, string::ParseError};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rope {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::E),
            "L" => Ok(Direction::W),
            "D" => Ok(Direction::S),
            "U" => Ok(Direction::N),
            _ => panic!(),
        }
    }
}

impl Rope {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn mov(&mut self, dir: Direction) {
        match dir {
            Direction::E => self.x += 1,
            Direction::W => self.x -= 1,
            Direction::S => self.y -= 1,
            Direction::N => self.y += 1,
            Direction::NE => {
                self.mov(Direction::N);
                self.mov(Direction::E);
            }
            Direction::NW => {
                self.mov(Direction::N);
                self.mov(Direction::W);
            }
            Direction::SE => {
                self.mov(Direction::S);
                self.mov(Direction::E);
            }
            Direction::SW => {
                self.mov(Direction::S);
                self.mov(Direction::W);
            }
        };
    }

    fn dist(&self, other: &Self) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }

    /// return the direction for `other` to move based on the position of `self`
    fn compare(&self, other: &Self) -> Option<Direction> {
        // single diagonal or same row
        if self.dist(other) <= 2 {
            return None;
        }

        if self.x == other.x {
            if self.y > other.y {
                return Some(Direction::N);
            } else {
                return Some(Direction::S);
            }
        }
        if self.y == other.y {
            if self.x > other.x {
                return Some(Direction::E);
            } else {
                return Some(Direction::W);
            }
        }

        if self.x > other.x {
            if self.y > other.y {
                return Some(Direction::NE);
            } else {
                return Some(Direction::SE);
            }
        } else {
            if self.y > other.y {
                return Some(Direction::NW);
            } else {
                return Some(Direction::SW);
            }
        }
    }
}

pub fn part_a(input: &str) -> usize {
    let mut head = Rope::new(0, 0);
    let mut tail = Rope::new(0, 0);
    let mut counter = HashSet::new();
    counter.insert((tail.x, tail.y));

    for line in input.lines() {
        let dm: Vec<_> = line.split_whitespace().collect();
        let dir = Direction::from_str(dm[0]).unwrap();
        let steps = usize::from_str(dm[1]).unwrap();
        for _ in 0..steps {
            head.mov(dir);
            if let Some(dir) = head.compare(&tail) {
                tail.mov(dir);
            }
            counter.insert((tail.x, tail.y));
        }
    }
    counter.len()
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

    // #[test]
    // fn part_b_sample() {
    //     assert_eq!(super::part_b(include_str!("sample.txt")), 36);
    // }

    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(include_str!("input.txt")), 0);
    // }
}
