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
    register_values: Vec<isize>,
    signal_strength: Vec<isize>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register: 1,
            clock: 1,
            register_values: Vec::new(),
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
            self.register_values.push(self.register);
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
    let start = 20;
    let step = 40;

    let mut cpu = CPU::new();
    let mut stack = Instruction::build(input);
    while !stack.is_empty() {
        cpu.process(stack.pop().unwrap());
    }

    let cycles = start..cpu.signal_strength.len();
    cycles
        .step_by(step)
        .map(|i| cpu.signal_strength[i - 1])
        .sum()
}

fn render_crt(register_values: Vec<isize>) -> String {
    let step = 40;
    let sprite_size = 3;

    let mut result = String::new();
    for (cycle, register) in register_values.iter().enumerate() {
        let sprite_start = register;
        let sprite_end = sprite_start + sprite_size - 1;
        let crt_position: isize = (cycle % step) as isize + 1;

        if sprite_start <= &crt_position && crt_position <= sprite_end {
            result.push('#');
        } else {
            result.push('.');
        }

        if crt_position as usize == step {
            result.push('\n');
        }
    }
    result
}

pub fn part_b(input: &str) -> String {
    let mut cpu = CPU::new();
    let mut stack = Instruction::build(input);
    while !stack.is_empty() {
        cpu.process(stack.pop().unwrap());
    }
    render_crt(cpu.register_values)
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

    #[test]
    fn part_b_sample() {
        assert_eq!(
            super::part_b(include_str!("sample.txt")),
            String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            ),
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(
            super::part_b(include_str!("input.txt")),
            String::from(
                "####..##...##....##.####...##.####.#....
...#.#..#.#..#....#....#....#.#....#....
..#..#....#.......#...#.....#.###..#....
.#...#.##.#.......#..#......#.#....#....
#....#..#.#..#.#..#.#....#..#.#....#....
####..###..##...##..####..##..#....####.
"
            ),
        );
    }
}
