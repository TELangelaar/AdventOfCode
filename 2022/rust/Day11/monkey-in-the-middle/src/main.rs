#![feature(int_roundings)]
use ::lending_iterator::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, one_of},
    combinator::map_res,
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};
use std::collections::VecDeque;

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        )
    });

    println!("{result}");
    let mut notes = notes(&result).unwrap().1;

    for _ in 0..20 {
        play_round(&mut notes);
    }

    println!("{notes:?}");
}

fn play_round(notes: &mut Vec<Note>) {
    for i in 0..notes.len() {
        let mut current_monkey = notes[i].clone();
        while let Some(item) = current_monkey.starting_items.pop_front() {
            let mut worry_level = match current_monkey.operation {
                Operation::Add(ref val) => item + val,
                Operation::Multiply(ref val) => item * val,
                Operation::Quadratic => item * item,
            };
            worry_level = worry_level.div_floor(3);
            let is_divisible_by = match current_monkey.test {
                Test::DivisibleBy(val) => worry_level % val == 0,
            };

            if is_divisible_by {
                let mut other_monkey = notes[current_monkey.if_true as usize].clone();
                other_monkey.starting_items.push_back(item);
                notes[current_monkey.if_true as usize] = other_monkey;
            } else {
                let mut other_monkey = notes[current_monkey.if_false as usize].clone();
                other_monkey.starting_items.push_back(item);
                notes[current_monkey.if_false as usize] = other_monkey;
            }
        }
        notes[i] = current_monkey;
    }
}

#[derive(Debug, Clone)]
struct Note {
    monkey: u32,
    starting_items: VecDeque<u32>,
    operation: Operation,
    test: Test,
    if_true: u32,
    if_false: u32,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Quadratic,
}

#[derive(Debug, Clone)]
enum Test {
    DivisibleBy(u32),
}

fn note(input: &str) -> IResult<&str, Note> {
    let (input, monkey) = preceded(tag("Monkey "), terminated(complete::u32, tag(":")))(input)?;

    let (input, starting_items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), complete::u32),
    )(input.trim_start())?;

    let (input, (operation, amount)) = preceded(
        tag("Operation: new = old "),
        separated_pair(one_of("*+"), tag(" "), alt((tag("old"), digit1))),
    )(input.trim_start())?;

    let (input, test) = preceded(
        tag("Test: divisible by "),
        map_res(digit1, str::parse).map(|digit| Test::DivisibleBy(digit)),
    )(input.trim_start())?;

    let (input, if_true) =
        preceded(tag("If true: throw to monkey "), complete::u32)(input.trim_start())?;

    let (input, if_false) =
        preceded(tag("If false: throw to monkey "), complete::u32)(input.trim_start())?;

    let operation = match operation {
        '*' => match amount {
            "old" => Operation::Quadratic,
            val => Operation::Multiply(val.parse().unwrap()),
        },
        '+' => Operation::Add(amount.parse().unwrap()),
        _ => panic!("this shouldn't happen"),
    };

    Ok((
        input,
        Note {
            monkey,
            starting_items: VecDeque::from_iter(starting_items),
            operation,
            test,
            if_true,
            if_false,
        },
    ))
}

fn notes(input: &str) -> IResult<&str, Vec<Note>> {
    let (input, notes) = separated_list1(count(line_ending, 2), note)(input)?;

    Ok((input, notes))
}
