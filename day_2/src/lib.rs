use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => panic!("Problem parsing hand shape: {:?}", s),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => panic!("Problem parsing outcome: {:?}", s),
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut score: i64 = 0;
    for line in input.trim().split("\n") {
        let mut split = line.split(" ");
        let theirs = split.next().unwrap().parse::<Shape>().unwrap();
        let ours = split.next().unwrap().parse::<Shape>().unwrap();

        let outcome = match theirs {
            Shape::Rock => match ours {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Lose,
            },
            Shape::Paper => match ours {
                Shape::Rock => Outcome::Lose,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Scissors => match ours {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Lose,
                Shape::Scissors => Outcome::Draw,
            },
        };
        score = score + outcome as i64 + ours as i64;
    }
    score
}

pub fn part_b(input: &str) -> i64 {
    let mut score: i64 = 0;
    for line in input.trim().split("\n") {
        let mut split = line.split(" ");
        let theirs = split.next().unwrap().parse::<Shape>().unwrap();
        let outcome = split.next().unwrap().parse::<Outcome>().unwrap();

        let ours = match theirs {
            Shape::Rock => match outcome {
                Outcome::Win => Shape::Paper,
                Outcome::Draw => Shape::Rock,
                Outcome::Lose => Shape::Scissors,
            },
            Shape::Paper => match outcome {
                Outcome::Win => Shape::Scissors,
                Outcome::Draw => Shape::Paper,
                Outcome::Lose => Shape::Rock,
            },
            Shape::Scissors => match outcome {
                Outcome::Win => Shape::Rock,
                Outcome::Draw => Shape::Scissors,
                Outcome::Lose => Shape::Paper,
            },
        };
        score = score + outcome as i64 + ours as i64;
    }
    score
}
#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 15);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 13924);
    }

    #[test]
    fn part_b_sample() {
        assert_eq!(super::part_b(include_str!("sample.txt")), 12);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 13448);
    }
}
