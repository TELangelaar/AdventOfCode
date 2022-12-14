use itertools::Itertools;
use std::{collections::HashMap, process};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    let result = result.replace("\r", ""); // change CRLF to LF
    let (layout, instructions) = result.split_once("\n\n").unwrap();

    let mut sc = StartingCrates::parse(layout);
    let instructions = Instruction::parse(instructions);
    println!("{sc:?}");

    let result = solve_part2(&mut sc, instructions);
    println!("{result}");
}

fn solve_part2(starting_crates: &mut StartingCrates, instructions: Vec<Instruction>) -> String {
    println!("{instructions:?}");
    for instruction in instructions {
        starting_crates.move_crates_part2(instruction);
    }
    println!("{starting_crates:?}");

    let mut result = String::new();
    for key in starting_crates.data.clone().keys().to_owned().sorted() {
        let case = starting_crates.data.get_mut(&key).unwrap();
        result.push(case.pop().unwrap_or(' '));
    }

    let result = result.replace(" ", "");
    result
}

fn solve_part1(starting_crates: &mut StartingCrates, instructions: Vec<Instruction>) -> String {
    println!("{instructions:?}");
    for instruction in instructions {
        starting_crates.move_crates_part1(instruction);
    }
    println!("{starting_crates:?}");

    let mut result = String::new();
    for key in starting_crates.data.clone().keys().to_owned().sorted() {
        let case = starting_crates.data.get_mut(&key).unwrap();
        result.push(case.pop().unwrap_or(' '));
    }

    let result = result.replace(" ", "");
    result
}

#[derive(Debug)]
struct StartingCrates {
    data: HashMap<usize, Vec<char>>,
}

impl StartingCrates {
    fn new() -> StartingCrates {
        StartingCrates {
            data: HashMap::new(),
        }
    }

    fn move_crates_part1(&mut self, instruction: Instruction) {
        let from = instruction.from;
        let to = instruction.to;
        let amount = instruction.amount_of_crates_to_move;
        println!("[starting] Moving {amount} from {from} to {to}");

        let mut stack_from = self.data.remove(&(instruction.from as usize)).unwrap();
        println!("removed {stack_from:?}");
        let mut stack_to = self.data.remove(&(instruction.to as usize)).unwrap();
        println!("removed {stack_to:?}");
        for _ in 0..instruction.amount_of_crates_to_move {
            let case = stack_from.pop().unwrap();
            println!("popped {case}");
            stack_to.push(case);
        }

        println!("inserting {stack_from:?}");
        self.data.insert(instruction.from as usize, stack_from);
        println!("inserting {stack_to:?}");
        self.data.insert(instruction.to as usize, stack_to);
    }

    fn move_crates_part2(&mut self, instruction: Instruction) {
        let from = instruction.from;
        let to = instruction.to;
        let amount = instruction.amount_of_crates_to_move;
        println!("[starting] Moving {amount} from {from} to {to}");

        let mut stack_from = self.data.remove(&(instruction.from as usize)).unwrap();
        println!("removed {stack_from:?}");
        let mut stack_to = self.data.remove(&(instruction.to as usize)).unwrap();
        println!("removed {stack_to:?}");

        let mut intermediate_stack = Vec::new();
        for _ in 0..instruction.amount_of_crates_to_move {
            let case = stack_from.pop().unwrap();
            println!("popped {case}");
            intermediate_stack.push(case);
        }

        intermediate_stack.reverse();
        for item in intermediate_stack {
            stack_to.push(item);
        }

        println!("inserting {stack_from:?}");
        self.data.insert(instruction.from as usize, stack_from);
        println!("inserting {stack_to:?}");
        self.data.insert(instruction.to as usize, stack_to);
    }

    fn parse(layout: &str) -> StartingCrates {
        let (stacks_str, platforms) = layout.rsplit_once('\n').unwrap();
        let platforms: Vec<&str> = platforms.split_whitespace().collect();

        let mut sc = StartingCrates::new();
        for platform in platforms {
            let key = platform.parse().unwrap();
            let vec: Vec<char> = Vec::new();
            sc.data.insert(key, vec);
        }

        for line in stacks_str.lines().rev() {
            for (idx, chunk) in line.chars().chunks(4).into_iter().enumerate() {
                let stack = sc.data.get_mut(&(idx + 1)).unwrap();
                for char in chunk {
                    if char.is_whitespace() {
                        break;
                    } else if char != '[' && char != ']' {
                        stack.push(char);
                    }
                }
            }
        }

        sc
    }
}

#[derive(Debug)]
struct Instruction {
    amount_of_crates_to_move: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn parse(instructions: &str) -> Vec<Instruction> {
        let mut result: Vec<Instruction> = Vec::new();

        let lines = instructions.lines();
        let mut line_result: Vec<u32> = Vec::new();
        for line in lines {
            for char in line.chars() {
                if char.is_digit(10) {
                    let num = char.to_digit(10).unwrap();
                    line_result.push(num);
                }
            }

            if line_result.len() == 4 {
                let mut first = line_result[0].to_string();
                first.push_str(line_result[1].to_string().as_str());
                let amount = first.parse().unwrap();
                let from = line_result[2];
                let to = line_result[3];
                line_result.clear();

                line_result.push(amount);
                line_result.push(from);
                line_result.push(to);
            }

            result.push(Instruction {
                amount_of_crates_to_move: line_result[0],
                from: line_result[1],
                to: line_result[2],
            });
            line_result.clear();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, Instruction, StartingCrates};

    #[test]
    fn move_crates_shouldmovesuccesfully_whenvalidinstructions() {
        let mut sc = StartingCrates::new();
        sc.data.insert(1, vec!['A', 'B', 'C']);
        sc.data.insert(2, vec!['D', 'E', 'F', 'G']);
        sc.data.insert(3, vec!['H', 'I']);

        let i1 = Instruction {
            amount_of_crates_to_move: 3,
            from: 1,
            to: 2,
        };
        let i2 = Instruction {
            amount_of_crates_to_move: 7,
            from: 2,
            to: 3,
        };
        let i3 = Instruction {
            amount_of_crates_to_move: 9,
            from: 3,
            to: 1,
        };
        let i4 = Instruction {
            amount_of_crates_to_move: 9,
            from: 1,
            to: 3,
        };
        let instructions = vec![i1, i2, i3, i4];

        assert_eq!("D", solve_part1(&mut sc, instructions));
    }
}
