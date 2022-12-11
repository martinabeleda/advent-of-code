use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    string::ParseError,
};

#[derive(Debug)]
enum Operation {
    Plus,
    Multiply,
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operation::Plus,
            "*" => Operation::Multiply,
            _ => panic!(),
        })
    }
}

#[derive(Debug)]
enum Term {
    Old,
    Val(usize),
}

impl FromStr for Term {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Term::Old,
            _ => Term::Val(s.parse::<usize>().unwrap()),
        })
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    left: Term,
    operation: Operation,
    right: Term,
    divisor: usize,
    t_monkey: usize,
    f_monkey: usize,
    inspect_count: usize,
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a: Vec<&str> = s.lines().collect();
        let items = a[1]
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        let op: Vec<&str> = a[2]
            .trim()
            .strip_prefix("Operation: new = ")
            .unwrap()
            .split(" ")
            .collect();
        let left = op[0].parse::<Term>().unwrap();
        let operation = op[1].parse::<Operation>().unwrap();
        let right = op[2].parse::<Term>().unwrap();

        let divisor: usize = a[3]
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let t_monkey: usize = a[4]
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let f_monkey: usize = a[5]
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(Self {
            items,
            left,
            operation,
            right,
            divisor,
            t_monkey,
            f_monkey,
            inspect_count: 0,
        })
    }
}

pub fn part_a(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .trim()
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut output: HashMap<usize, VecDeque<usize>> = HashMap::new();
            while let Some(item) = monkey.items.pop_front() {
                monkey.inspect_count += 1;
                let left = match monkey.left {
                    Term::Old => item,
                    Term::Val(x) => x,
                };
                let right = match monkey.right {
                    Term::Old => item,
                    Term::Val(x) => x,
                };
                let worry = match monkey.operation {
                    Operation::Plus => left + right,
                    Operation::Multiply => left * right,
                };

                let worry = worry / 3;
                if worry % monkey.divisor == 0 {
                    output
                        .entry(monkey.t_monkey)
                        .or_insert_with(|| VecDeque::new().into())
                        .push_back(worry);
                } else {
                    output
                        .entry(monkey.f_monkey)
                        .or_insert_with(|| VecDeque::new().into())
                        .push_back(worry);
                }
            }

            for (key, mut monkey) in output.into_iter() {
                monkeys[key].items.append(&mut monkey)
            }
        }
    }
    let mut res: Vec<usize> = monkeys.into_iter().map(|m| m.inspect_count).collect();
    res.sort();
    res.reverse();
    res.iter().take(2).fold(1, |acc, x| acc * x)
}

pub fn part_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 10605);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 76728);
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
