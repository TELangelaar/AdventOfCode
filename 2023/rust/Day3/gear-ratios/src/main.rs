use std::ops::Range;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take},
    character::complete::{digit1, line_ending, one_of},
    combinator::{map, peek},
    multi::{many_till, separated_list1},
    IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        )
    });

    // let schematic = EngineSchematic::parse(&result).unwrap().1;
    let schematic = Schematic::parse(&result).unwrap().1;
    // dbg!(&schematic);

    // part 2
    let part2 = solve_part1(&schematic);
}

fn solve_part1<'a>(schematic: &'a Schematic<'a>) -> Vec<u32> {
    let mut gear_ratios = vec![];
    for (i, row) in schematic.pieces.iter().enumerate() {
        for (j, piece) in row.iter().enumerate() {
            match piece.part {
                Part::Number(_) => (),
                _ => continue,
            }
            let is_part = is_part(i, schematic, j, row, piece);
            
            let mut gear_ratio = 0;
            if is_part {
               match piece.part {
                Part::Number(val) => todo!(),
                _ => panic!()
               }
            } else {

            }
        }
    }

    gear_ratios
}

fn is_part(i: usize, schematic: &Schematic<'_>, j: usize, row: &Vec<Piece<'_>>, piece: &Piece<'_>) -> bool {
    let mut is_part = false;

    let above = match i {
        0 => None,
        _ => schematic.pieces.get(i - 1),
    };

    let below = schematic.pieces.get(i + 1);
    let left = if j == 0 { None } else { row.get(j - 1) };
    let right = row.get(j + 1);

    let range_start = match piece.col_range.start {
        0 => 0,
        _ => piece.col_range.start - 1,
    };
    let range = (range_start)..(piece.col_range.end + 1);

    if let Some(row_above) = above {
        if row_above
            .iter()
            .filter(|&piece_compare| match piece_compare.part {
                Part::Symbol => true,
                _ => false,
            })
            .any(|piece_compare| range.contains(&piece_compare.col_range.start))
        {
            println!("Found part using row above: {:#?}", piece);
            is_part = true;
        };
    }
    if let Some(row_below) = below {
        if row_below
            .iter()
            .filter(|&piece_compare| match piece_compare.part {
                Part::Symbol => true,
                _ => false,
            })
            .any(|piece_compare| range.contains(&piece_compare.col_range.start))
        {
            println!("Found part using row below: {:#?}", piece);
            is_part = true;
        };
    }
    if let Some(piece_left) = left {
        match piece_left.part {
            Part::Symbol => {
                if range.contains(&piece_left.col_range.start) {
                    println!("Found part using left: {:#?}", piece);
                    is_part = true;
                }
            }
            _ => (),
        }
    }
    if let Some(piece_right) = right {
        match piece_right.part {
            Part::Symbol => {
                if range.contains(&piece_right.col_range.start) {
                    println!("Found part using right: {:#?}", piece);
                    is_part = true;
                }
            }
            _ => (),
        }
    }

    is_part
}

#[derive(Debug)]
struct Schematic<'a> {
    pieces: Vec<Vec<Piece<'a>>>,
}

#[derive(Debug)]
struct Piece<'a> {
    part: Part<'a>,
    col_range: Range<usize>,
}

#[derive(Debug)]
enum Part<'a> {
    Number(&'a str),
    Period,
    Symbol,
    Star,
}

impl Schematic<'_> {
    fn parse(input: &str) -> IResult<&str, Schematic> {
        let (input, pieces) = separated_list1(line_ending, Schematic::parse_pieces)(input)?;

        Ok((input, Schematic { pieces }))
    }

    fn parse_pieces(input: &str) -> IResult<&str, Vec<Piece>> {
        let (input, (parts, _)) = many_till(
            alt((
                map(one_of("!@#$%^&()_-=+`~/:;,"), |_| Part::Symbol),
                map(tag("."), |_| Part::Period),
                map(tag("*"), |_| Part::Star),
                map(digit1, |s: &str| Part::Number(s)),
                // map(take(1usize), |s: &str| match s {
                //     "." => Part::Period,
                //     _ => Part::Symbol,
                // }),
            )),
            peek(line_ending),
        )(input)?;

        let mut i = 0;
        let pieces = parts
            .into_iter()
            .map(|part| match part {
                Part::Number(val) => {
                    let piece = Piece {
                        part: Part::Number(val),
                        col_range: i..(i + val.len()),
                    };
                    i += val.len();
                    piece
                }
                part => {
                    let piece = Piece {
                        part,
                        col_range: i..(i + 1),
                    };
                    i += 1;
                    piece
                }
            })
            .collect::<Vec<Piece>>();

        Ok((input, pieces))
    }
}
