use std::collections::HashSet;

use nom::{IResult, branch::alt, sequence::preceded, character::complete::{alpha1, digit1, line_ending}, bytes::complete::tag, multi::separated_list1, Parser};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let input = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        )
    });

    println!("{}", input);

    let volcano = volcano(&input, 30).unwrap().1;
    println!("{:?}", volcano);

}

#[derive(Debug, Eq, Hash)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    next_valves: Vec<&'a str>
}

impl PartialEq for Valve<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name 
    }
}

#[derive(Debug)]
struct Volcano<'a> {
    total_minutes: usize,
    current_minute: usize,
    total_flow_rate: u32,
    open_valves: Vec<&'a str>,
    valves: HashSet<Valve<'a>>
}

fn valve(input: &str) -> IResult<&str, Valve> {
    let (input, name) = preceded(tag("Valve "), alpha1)(input)?;
    let (input, flow_rate) = preceded(tag("rate="), digit1.map(|digit: &str| digit.parse::<u32>().unwrap()))(input)?;
    let (input, next_valves) = alt((
                                                    preceded(tag("to valve "), separated_list1(tag(", "), alpha1)), 
                                                    preceded(tag("to valves "), separated_list1(tag(", "), alpha1))
                                                ))(input)?;

    Ok((input, Valve {
        name,
        flow_rate,
        next_valves
    }))
}

fn volcano(input: &str, total_minutes: usize) -> IResult<&str, Volcano> {
    let (input, valves_vec) = separated_list1(line_ending, valve)(input)?;

    let mut valves = HashSet::new();
    for valve in valves_vec {
        let name = valve.name;
        if !valves.insert(valve) {
            panic!("HashSet<Valve> already contains {}", name);
        }
    }

    Ok((input, Volcano {
        total_minutes,
        current_minute: 0,
        total_flow_rate: 0,
        open_valves: vec![],
        valves
    }))
}