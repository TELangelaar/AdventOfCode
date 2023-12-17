use std::{collections::HashMap, iter::zip, ops::Range, time::Instant};

use nom::{
    branch::{alt, permutation, Permutation},
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::{many1, many_till, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
Time:      7  15   30
Distance:  9  40  200
",
        )
    });

    // Part 1
    let races = parse_races(&result).unwrap().1;
    let total = races
        .into_iter()
        .fold(1usize, |acc, race| acc * race.get_winning_posibilities());
    println!("Part1 answer: {:#?}", total);
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn get_winning_posibilities(&self) -> usize {
        let mut won = 0usize;
        for i in 0..self.time {
            if i * (self.time - i) > self.distance {
                won += 1;
            }
        }

        won
    }
}

fn parse_line<'a>(preceded_tag: &str, input: &'a str) -> IResult<&'a str, Vec<usize>> {
    let (input, (digits, _)) = preceded(
        tag(preceded_tag),
        many_till(
            preceded(
                many1(tag(" ")),
                map(digit1, |s: &str| s.parse::<usize>().unwrap()),
            ),
            line_ending,
        ),
    )(input)?;

    Ok((input, digits))
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = parse_line("Time:", input)?;
    let (input, distances) = parse_line("Distance:", input)?;

    Ok((
        input,
        zip(times, distances)
            .map(|(time, distance)| Race { time, distance })
            .collect::<Vec<Race>>(),
    ))
}
