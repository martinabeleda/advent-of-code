use std::str::FromStr;
use std::{cmp::Ordering, cmp::PartialOrd, string::ParseError};

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
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

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let result = match (self, other) {
            (Shape::Rock, Shape::Rock) => Ordering::Equal,
            (Shape::Rock, Shape::Paper) => Ordering::Less,
            (Shape::Rock, Shape::Scissors) => Ordering::Greater,
            (Shape::Scissors, Shape::Scissors) => Ordering::Equal,
            (Shape::Scissors, Shape::Paper) => Ordering::Greater,
            (Shape::Scissors, Shape::Rock) => Ordering::Less,
            (Shape::Paper, Shape::Paper) => Ordering::Equal,
            (Shape::Paper, Shape::Rock) => Ordering::Greater,
            (Shape::Paper, Shape::Scissors) => Ordering::Less,
        };
        Some(result)
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

fn outcome_from_game(our: &Shape, their: &Shape) -> Outcome {
    let outcome = if our == their {
        Outcome::Draw
    } else if our < their {
        Outcome::Lose
    } else {
        Outcome::Win
    };
    outcome
}

pub fn part_a(input: &str) -> usize {
    let mut score: usize = 0;
    for line in input.trim().split("\n") {
        let mut split = line.split(" ");
        let theirs = split.next().unwrap().parse::<Shape>().unwrap();
        let ours = split.next().unwrap().parse::<Shape>().unwrap();

        let outcome = outcome_from_game(&ours, &theirs);
        score = score + outcome as usize + ours.score();
    }
    score
}

pub fn part_b(input: &str) -> usize {
    let mut score: usize = 0;
    for line in input.trim().split("\n") {
        let mut split = line.split(" ");
        let theirs = split.next().unwrap().parse::<Shape>().unwrap();
        let outcome = split.next().unwrap().parse::<Outcome>().unwrap();

        let ours = match (theirs, &outcome) {
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        };
        score = score + outcome as usize + ours.score();
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
