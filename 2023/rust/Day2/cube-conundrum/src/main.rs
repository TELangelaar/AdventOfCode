use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::preceded,
    IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        )
    });

    let bag = Cubes {
        red: Some(12),
        green: Some(13),
        blue: Some(14)
    };

    // Part 1
    let games = parse_games(&result).unwrap().1;
    let possible_games = games.into_iter().filter(|game| !game.subsets.iter().any(
        |cubes| {
            match (cubes.red, cubes.green, cubes.blue) {
                (None, None, None) => panic!(),
                (None, None, Some(blue)) => blue > bag.blue.unwrap(),
                (None, Some(green), None) => green > bag.green.unwrap(),
                (None, Some(green), Some(blue)) => blue > bag.blue.unwrap() || green > bag.green.unwrap(),
                (Some(red), None, None) => red > bag.red.unwrap(),
                (Some(red), None, Some(blue)) => red > bag.red.unwrap() || blue > bag.blue.unwrap(),
                (Some(red), Some(green), None) => red > bag.red.unwrap() || green > bag.green.unwrap(),
                (Some(red), Some(green), Some(blue)) => red > bag.red.unwrap() || green > bag.green.unwrap() || blue > bag.blue.unwrap(),
            }
        }
    ));
    println!("{:#?}", possible_games.map(|x| x.id).sum::<u32>());

    // Part 2
    let games = parse_games(&result).unwrap().1;
    let sets: Vec<Cubes> = games.into_iter().map(|game| game.subsets.iter().fold(Cubes {red: Some(0), green: Some(0), blue: Some(0)}, |mut acc, cubes| {
        if let Some(red) = cubes.red {
            if red > acc.red.unwrap() {
            acc.red = cubes.red;
            }
        }
        if let Some(green) = cubes.green {
            if green > acc.green.unwrap() {
            acc.green = cubes.green;
            }
        }
        if let Some(blue) = cubes.blue {
            if blue > acc.blue.unwrap() {
            acc.blue = cubes.blue;
            }
        }
        acc
    })).collect();
    let powers: Vec<u64> = sets.into_iter().map(|cubes| (cubes.red.unwrap() * cubes.green.unwrap() * cubes.blue.unwrap()) as u64).collect();
    println!("{:#?}", powers.into_iter().sum::<u64>());
}

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Cubes>,
}

#[derive(Debug)]
struct Cubes {
    red: Option<usize>,
    blue: Option<usize>,
    green: Option<usize>,
}

fn subset(input: &str) -> IResult<&str, Cubes> {
    let mut cubes = Cubes {
        red: None,
        blue: None,
        green: None,
    };

    let (input, _) = separated_list0(
        tag(", "),
        map(
            permutation((
                map(digit1, |s: &str| s.parse::<usize>().unwrap()),
                tag(" "),
                alpha1,
            )),
            |(amount, _, color)| match color {
                "green" => match cubes.green {
                    None => cubes.green = Some(amount),
                    Some(x) => cubes.green = Some(x + amount),
                },
                "red" => match cubes.red {
                    None => cubes.red = Some(amount),
                    Some(x) => cubes.red = Some(x + amount),
                },
                "blue" => match cubes.blue {
                    None => cubes.blue = Some(amount),
                    Some(x) => cubes.blue = Some(x + amount),
                },
                _ => panic!(),
            },
        ),
    )(input)?;

    Ok((input, cubes))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), map(digit1, |s: &str| s.parse().unwrap()))(input)?;
    let (input, subsets) = preceded(tag(": "), separated_list1(tag("; "), subset))(input)?;

    let game = Game { id, subsets };

    Ok((input, game))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;

    Ok((input, games))
}
