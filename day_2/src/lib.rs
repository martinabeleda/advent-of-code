use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
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
            _ => panic!("Problem parsing strategy: {:?}", s),
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut score: i64 = 0;
    for line in input.trim().split("\n") {
        let mut split = line.split(" ");
        let opp_strat = split.next().unwrap().parse::<HandShape>().unwrap();
        let my_strat = split.next().unwrap().parse::<HandShape>().unwrap();

        let shape_score = match my_strat {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        };

        let result = match opp_strat {
            HandShape::Rock => match my_strat {
                HandShape::Rock => 3,
                HandShape::Paper => 6,
                HandShape::Scissors => 0,
            },
            HandShape::Paper => match my_strat {
                HandShape::Rock => 0,
                HandShape::Paper => 3,
                HandShape::Scissors => 6,
            }
            HandShape::Scissors => match my_strat {
                HandShape::Rock => 6,
                HandShape::Paper => 0,
                HandShape::Scissors => 3,
            }
        };
        score = score + result + shape_score;
    };
    score
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 13924);
    }
}
