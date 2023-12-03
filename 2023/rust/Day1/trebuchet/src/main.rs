use std::iter::zip;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, alphanumeric1, digit1, line_ending},
    combinator::{map, map_parser},
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        )
    });

    // PART 1
    // let values = calibration_values_digits(&result).unwrap().1;
    // let sum: u32 = values.iter().sum();
    // println!("{:#?}", sum);

    // PART 2
    let firsts = calibration_values_spelled_first(&result).unwrap().1;
    let input_reversed = result.chars().rev().collect::<String>();
    let lasts = calibration_values_spelled_last(&input_reversed).unwrap().1;

    let mut result: Vec<u32> = vec![];
    for (first, last) in zip(firsts, lasts) {
        result.push((first.to_owned() + last).parse().unwrap());
    }
    println!("{:#?}", result.iter().sum::<u32>());
}

fn calibration_values_spelled_first(input: &str) -> IResult<&str, Vec<&str>> {
    let wanted_tokens = alt((
        map_parser(take(1usize), digit1),
        map(tag("one"), |_| "1"),
        map(tag("two"), |_| "2"),
        map(tag("three"), |_| "3"),
        map(tag("four"), |_| "4"),
        map(tag("five"), |_| "5"),
        map(tag("six"), |_| "6"),
        map(tag("seven"), |_| "7"),
        map(tag("eight"), |_| "8"),
        map(tag("nine"), |_| "9"),
        map(map_parser(take(1usize), alpha1), |_| "0"),
    ));

    let (input, first) = separated_list1(line_ending, many1(wanted_tokens))(input)?;

    let mut result = vec![];
    for row in first {
        let filtered = row
            .into_iter()
            .filter(|&item| item != "0")
            .collect::<Vec<&str>>();
        result.push(filtered.first().unwrap().to_owned());
    }

    Ok((input, result))
}

fn calibration_values_spelled_last(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = line_ending(input)?;
    let wanted_tokens = alt((
        map_parser(take(1usize), digit1),
        map(tag("eno"), |_| "1"),
        map(tag("owt"), |_| "2"),
        map(tag("eerht"), |_| "3"),
        map(tag("ruof"), |_| "4"),
        map(tag("evif"), |_| "5"),
        map(tag("xis"), |_| "6"),
        map(tag("neves"), |_| "7"),
        map(tag("thgie"), |_| "8"),
        map(tag("enin"), |_| "9"),
        map(map_parser(take(1usize), alpha1), |_| "0"),
    ));

    let (input, first) = separated_list1(line_ending, many1(wanted_tokens))(input)?;

    let mut result = vec![];
    for row in first {
        let filtered = row
            .into_iter()
            .filter(|&item| item != "0")
            .collect::<Vec<&str>>();
        result.push(filtered.first().unwrap().to_owned());
    }
    result.reverse();

    Ok((input, result))
}

fn calibration_value_digits(input: &str) -> IResult<&str, u32> {
    let (input, chars) = alphanumeric1(input)?;

    let digits = chars
        .chars()
        .filter(|c| c.to_digit(10).is_some())
        .collect::<Vec<char>>();
    let result = digits.first().unwrap().to_string() + &digits.last().unwrap().to_string();

    Ok((input, result.parse().unwrap()))
}

fn calibration_values_digits(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, values) = separated_list1(line_ending, calibration_value_digits)(input)?;

    Ok((input, values))
}
