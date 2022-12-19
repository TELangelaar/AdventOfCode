use ::lending_iterator::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        )
    });

    println!("{result}");

    let instructions = instructions(&result).unwrap().1;
    println!("{instructions:?}");

    solve(&instructions, 10);
}

fn solve(instructions: &Vec<Instruction>, knots: u32) {
    let mut rope = Rope::new(knots);
    for instruction in instructions {
        rope.move_head(&instruction);
    }
    let final_positions = &rope.knots;
    println!("final_positions:\n{final_positions:?}");
    let result = rope.tail_positions;
    println!("tail_positions: {result:?}");
    let answ = result.len();
    println!("unique positions: {answ}");
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Position>,
    tail_positions: HashSet<Position>,
}

impl Rope {
    fn new(number_of_knots: u32) -> Rope {
        let mut knots = Vec::new();
        for _ in 0..number_of_knots {
            knots.push(Position { x: 0, y: 0 });
        }
        let tail_positions = HashSet::from([knots.last().unwrap().clone()]);
        Rope {
            knots,
            tail_positions,
        }
    }

    fn move_head(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Right => {
                self.knots[0].x += 1;
            }
            Instruction::Left => {
                self.knots[0].x -= 1;
            }
            Instruction::Up => {
                self.knots[0].y += 1;
            }
            Instruction::Down => {
                self.knots[0].y -= 1;
            }
        }

        let mut knots_windows = self.knots.windows_mut::<2>();
        while let Some([ref head, ref mut tail]) = knots_windows.next() {
            if !knot_is_connected(head, tail) {
                move_tail(tail, head);
            }
        }
        self.tail_positions
            .insert(self.knots.last().unwrap().clone());
    }
}

fn move_tail(tail: &mut Position, head: &Position) {
    let position_difference = calculate_cartesian_difference(tail, head);

    tail.x += match position_difference.x {
        x if i32::abs(x) > 1 => x / 2,
        x => x,
    };
    tail.y += match position_difference.y {
        y if i32::abs(y) > 1 => y / 2,
        y => y,
    };
}

fn calculate_cartesian_difference(tail: &Position, head: &Position) -> Position {
    Position {
        x: head.x - tail.x,
        y: head.y - tail.y,
    }
}

fn knot_is_connected(head: &Position, tail: &Position) -> bool {
    let (x_range, y_range) = get_vicinity(tail);

    x_range.into_iter().any(|x| x == head.x) && y_range.into_iter().any(|y| y == head.y)
}

fn get_vicinity(head: &Position) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let x_range = (head.x - 1)..=(head.x + 1);
    let y_range = (head.y - 1)..=(head.y + 1);

    (x_range, y_range)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Right,
    Left,
    Up,
    Down,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = one_of("RLUD")(input)?;

    let instruction = match direction {
        'R' => Instruction::Right,
        'L' => Instruction::Left,
        'U' => Instruction::Up,
        'D' => Instruction::Down,
        _ => panic!("Unexpected direction"),
    };

    Ok((input, instruction))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(
        line_ending,
        separated_pair(instruction, tag(" "), complete::u32),
    )(input)?;

    let instructions = instructions
        .iter()
        .flat_map(|(direction, amount)| vec![*direction; *amount as usize])
        .collect();

    Ok((input, instructions))
}
