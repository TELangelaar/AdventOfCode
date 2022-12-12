use std::{env, fmt::Debug, process};

use anyhow::{anyhow, Context, Result};
use io::loading;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let content = loading::read_file(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    /* let content = "\
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2"; */

    let dive = solve(&content).unwrap();
    println!("{dive:?}");

    let answer = dive.vertical * dive.horizontal;
    println!("{answer}");
}

pub fn solve(content: &str) -> anyhow::Result<Dive> {
    let mut dive = Dive::new();

    for line in content.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();

        let direction: &str = &split[0];
        let amount: i32 = split[1].parse().unwrap();
        dive.process_change(direction, amount);
    }

    Ok(dive)
}

#[derive(Debug)]
pub struct Dive {
    pub vertical: i32,
    pub horizontal: i32,
    pub aim: i32,
}

impl Dive {
    pub fn new() -> Dive {
        let vertical = 0;
        let horizontal = 0;
        let aim = 0;

        Dive {
            vertical,
            horizontal,
            aim,
        }
    }

    pub fn process_change(&mut self, direction: &str, amount: i32) {
        match direction {
            "forward" => {
                self.horizontal += amount;
                self.vertical += self.aim * amount;
            }
            "backward" => self.horizontal -= amount,
            "down" => {
                self.aim += amount;
            }
            "up" => {
                self.aim -= amount;
            }
            _ => {}
        }
    }
}
