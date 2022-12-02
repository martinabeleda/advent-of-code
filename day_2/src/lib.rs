use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for HandShape {
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
enum Strategy {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Strategy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => panic!("Problem parsing strategy: {:?}", s),
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut score: i64 = 0;
    for line in input.trim().split("\n") {
        let mut split = line.split(" ");
        let opp_shape = split.next().unwrap().parse::<HandShape>().unwrap();
        let my_shape = split.next().unwrap().parse::<HandShape>().unwrap();

        let strategy = match opp_shape {
            HandShape::Rock => match my_shape {
                HandShape::Rock => Strategy::Draw,
                HandShape::Paper => Strategy::Win,
                HandShape::Scissors => Strategy::Lose,
            },
            HandShape::Paper => match my_shape {
                HandShape::Rock => Strategy::Lose,
                HandShape::Paper => Strategy::Draw,
                HandShape::Scissors => Strategy::Win,
            },
            HandShape::Scissors => match my_shape {
                HandShape::Rock => Strategy::Win,
                HandShape::Paper => Strategy::Lose,
                HandShape::Scissors => Strategy::Draw,
            },
        };
        let shape_score = my_shape as i64;
        let result = strategy as i64;
        score = score + result + shape_score;
    }
    score
}

pub fn part_b(input: &str) -> i64 {
    let mut score: i64 = 0;
    for line in input.trim().split("\n") {
        let mut split = line.split(" ");
        let opp_shape = split.next().unwrap().parse::<HandShape>().unwrap();
        let my_strat = split.next().unwrap().parse::<Strategy>().unwrap();

        let my_shape = match opp_shape {
            HandShape::Rock => match my_strat {
                Strategy::Win => HandShape::Paper,
                Strategy::Draw => HandShape::Rock,
                Strategy::Lose => HandShape::Scissors,
            },
            HandShape::Paper => match my_strat {
                Strategy::Win => HandShape::Scissors,
                Strategy::Draw => HandShape::Paper,
                Strategy::Lose => HandShape::Rock,
            },
            HandShape::Scissors => match my_strat {
                Strategy::Win => HandShape::Rock,
                Strategy::Draw => HandShape::Scissors,
                Strategy::Lose => HandShape::Paper,
            },
        };

        let shape_score = my_shape as i64;
        let result = my_strat as i64;
        score = score + result + shape_score;
    }
    score
}
#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 13924);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 13448);
    }
}
