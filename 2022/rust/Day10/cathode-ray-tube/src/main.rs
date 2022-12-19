use itertools::Itertools;
use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        )
    });

    let instructions = instructions(&result).unwrap().1;
    println!("{instructions:?}");

    let (crt, answer) = solve(&instructions);
    println!("answer part1: {answer}");
    println!("{crt}");
}

fn solve(instructions: &Vec<Instruction>) -> (CRT, i32) {
    let mut values: Vec<i32> = vec![0, 1];

    let mut crt = CRT {
        pixels: String::new(),
        current_cycle: 0,
    };
    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                let last_value = values.last().unwrap().clone();
                run_one_cycle(last_value, 0, &mut values, &mut crt);
            }
            Instruction::Addx { value } => {
                let last_value = values.last().unwrap().clone();
                run_one_cycle(last_value, 0, &mut values, &mut crt);
                run_one_cycle(last_value, *value, &mut values, &mut crt);
            }
        };
    }

    let signal_strenghts: Vec<i32> = values
        .iter()
        .enumerate()
        .filter(|(i, _)| i.is_relevant_cycle())
        .map(|(i, item)| item * i as i32)
        .collect();

    (crt, signal_strenghts.iter().sum())
}

fn run_one_cycle(value: i32, new_value: i32, values: &mut Vec<i32>, crt: &mut CRT) {
    crt.add_pixel(&value);
    values.push(value + new_value);
}

struct CRT {
    pixels: String,
    current_cycle: i32,
}

impl CRT {
    fn add_pixel(&mut self, value: &i32) {
        if self.current_cycle % 40 == 0 {
            self.current_cycle = 0;
        }
        let pixel_range = value - 1..=value + 1;
        if pixel_range.contains(&self.current_cycle) {
            self.pixels.push('#');
        } else {
            self.pixels.push('.');
        }
        self.current_cycle += 1;
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pixels
                .chars()
                .chunks(40)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .join("\n")
        )
    }
}

trait CPU {
    fn is_relevant_cycle(&self) -> bool;
}

impl CPU for usize {
    fn is_relevant_cycle(&self) -> bool {
        if self % 40 == 20 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx { value: i32 },
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(
        line_ending,
        alt((
            tag("noop").map(|_| Instruction::Noop),
            preceded(tag("addx "), complete::i32).map(|num| Instruction::Addx { value: num }),
        )),
    )(input)?;

    Ok((input, instructions))
}
