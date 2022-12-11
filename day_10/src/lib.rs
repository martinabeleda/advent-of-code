use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl Instruction {
    fn build(input: &str) -> Vec<Instruction> {
        let mut stack: Vec<Instruction> = input
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect();
        stack.reverse();

        stack
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(" ") {
            None => Self::Noop,
            Some((ins, val)) => match ins {
                "addx" => Self::Addx(val.parse::<isize>().unwrap()),
                _ => panic!(),
            },
        })
    }
}

#[derive(Debug)]
struct CPU {
    register: isize,
    clock: usize,
    signal_strength: Vec<isize>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register: 1,
            clock: 1,
            signal_strength: Vec::new(),
        }
    }

    fn process(&mut self, i: Instruction) {
        // wait the required cycles
        let cycles = match i {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        };
        for _ in 0..cycles {
            self.signal_strength
                .push(self.clock as isize * self.register);
            self.clock += 1;
        }

        // execute the instruction
        match i {
            Instruction::Noop => {}
            Instruction::Addx(val) => {
                self.register += val;
            }
        };
    }
}

pub fn part_a(input: &str) -> isize {
    let mut cpu = CPU::new();
    let mut stack = Instruction::build(input);
    while !stack.is_empty() {
        cpu.process(stack.pop().unwrap());
    }

    (20..cpu.signal_strength.len())
        .step_by(40)
        .map(|i| cpu.signal_strength[i - 1])
        .sum()
}

pub fn part_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a(include_str!("sample.txt")), 13140);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 14040);
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
