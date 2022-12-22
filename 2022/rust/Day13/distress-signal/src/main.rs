#![feature(iter_intersperse)]

use std::{cmp::Ordering, fmt::Display, iter::zip};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{count, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        )
    });

    let mut pairs = packet_pairs(&result).unwrap().1;
    let mut indices = vec![];
    for (idx, pair) in pairs.iter().enumerate() {
        let left = &pair.left;
        let right = &pair.right;
        let result = solve_part1(left, right);

        if let Order::Right = result {
            indices.push(idx + 1);
        }
    }

    println!(
        "Sum of indices in right order: {}",
        indices.iter().sum::<usize>()
    );

    //Part 2
    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let mut packets = pairs
        .iter()
        .flat_map(|pair| vec![pair.left.clone(), pair.right.clone()].into_iter())
        .chain([packet_2.clone(), packet_6.clone()])
        .collect::<Vec<Packet>>();
    packets.sort();

    let decoder_key = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| *packet == &packet_2 || *packet == &packet_6)
        .map(|(idx, _)| idx + 1)
        .collect::<Vec<usize>>()
        .iter()
        .fold(1, |acc, x| acc * x);
    println!("Decoder key: {}", decoder_key);
}

fn solve_part1(left: &Packet, right: &Packet) -> Order {
    match left {
        Packet::List(nums_left) => match right {
            Packet::List(nums_right) => {
                for (left_packet, right_packet) in zip(nums_left, nums_right) {
                    let result = solve_part1(left_packet, right_packet);
                    match result {
                        Order::Continue => (),
                        Order::Right => return Order::Right,
                        Order::Wrong => return Order::Wrong,
                    }
                }
                if nums_left.len() < nums_right.len() {
                    return Order::Right;
                } else if nums_left.len() > nums_right.len() {
                    return Order::Wrong;
                } else {
                    return Order::Continue;
                }
            }
            Packet::Number(num_right) => {
                let list = Packet::List(vec![Packet::Number(*num_right)]);
                solve_part1(left, &list)
            }
        },
        Packet::Number(num_left) => match right {
            Packet::List(_) => {
                let list = Packet::List(vec![Packet::Number(*num_left)]);
                solve_part1(&list, right)
            }
            Packet::Number(num_right) => {
                if num_left < num_right {
                    return Order::Right;
                } else if num_left > num_right {
                    return Order::Wrong;
                } else {
                    return Order::Continue;
                }
            }
        },
    }
}

#[derive(Debug)]
enum Order {
    Continue,
    Right,
    Wrong,
}

#[derive(Debug, Clone)]
struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(u8),
}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::Number(r0)) => l0 == &vec![Packet::Number(*r0)],
            (Self::Number(l0), Self::List(r0)) => &vec![Packet::Number(*l0)] == r0,
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(&b),
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}
impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Packet::List(list) => format!(
                    "[{}]",
                    list.iter()
                        .map(|v| v.to_string())
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Packet::Number(num) => num.to_string(),
            }
        )
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        nom::character::complete::u8.map(|num| Packet::Number(num)),
    ))(input)
}

fn packet_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    let (input, packets) = separated_list1(
        count(line_ending, 2),
        separated_pair(packet, line_ending, packet).map(|(p1, p2)| Pair {
            left: p1,
            right: p2,
        }),
    )(input)?;

    Ok((input, packets))
}
