use std::{env, process};

use io::{loading, parsing};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let content = loading::read_file(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    /* let content = "\
    199
    200
    208
    210
    200
    207
    240
    269
    260
    263"; */

    let result = parsing::lines_to_vec(&content).unwrap();
    let answer = solve1(&result);
    let answer2 = solve2(&result);

    println!("{answer:?}");
    println!("{answer2:?}");
}

fn solve1(input: &Vec<u32>) -> u32 {
    let mut answer = 0;
    for n in 1..input.len() {
        if input[n] > input[n - 1] {
            answer += 1;
        }
    }

    answer
}

fn solve2(input: &Vec<u32>) -> u32 {
    let mut answer = 0;

    let end = input.len() - 3;
    for n in 0..end {
        let first_window = input[n] + input[n + 1] + input[n + 2];
        let second_window = input[n + 1] + input[n + 2] + input[n + 3];

        if second_window > first_window {
            answer += 1;
        }
    }

    answer
}
