use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::{many0, many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let args = &vec!["xinput.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        )
    });

    //Part1
    let cards = parse_cards(&result).unwrap().1;
    let sum = cards.iter().fold(0u32, |acc, card| {
        acc + card.card_numbers.iter().fold(0u32, |acc2, i| {
            if card.winning_numbers.contains(&i) {
                match acc2 {
                    0 => 1,
                    x => x * 2,
                }
            } else {
                acc2
            }
        })
    });
    println!("sum: {:#?}", sum);

    //Part2
    let mut cards_won = vec![];
    let mut lookup = vec![];
    for card in cards.iter() {
        cards_won.push(1); //index is card id

        let n_matching = card
            .card_numbers
            .iter()
            .filter(|num| card.winning_numbers.contains(num))
            .count();

        lookup.push(n_matching);
    }

    println!("Before: {:#?}", cards_won);
    for (i, value) in cards_won.iter().enumerate() {
        for j in (i + 1)..=(i + value) {
            *cards_won.get_mut(j).unwrap() += 1;
        }
    }

    let sum2: usize = cards_won.iter().sum();
    println!("{:#?}", sum2);
    dbg!(cards_won);
    dbg!(lookup);
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;

    Ok((input, cards))
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = terminated(tag("Card"), many1(tag(" ")))(input)?;
    let (input, card_index) = map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)?;
    let (input, _) = terminated(tag(":"), many0(tag(" ")))(input)?;

    let (input, winning_numbers) = separated_list1(
        many1(tag(" ")),
        map(digit1, |s: &str| s.parse::<u32>().unwrap()),
    )(input)?;
    let (input, _) = tag(" |")(input)?;
    let (input, card_numbers) = many1(preceded(
        many1(tag(" ")),
        map(digit1, |s: &str| s.parse::<u32>().unwrap()),
    ))(input)?;

    Ok((
        input,
        Card {
            id: card_index,
            winning_numbers,
            card_numbers,
        },
    ))
}
