use std::ops::Range;

use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete::{digit1, line_ending},
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
.664.598..",
        )
    });

    // let schematic = EngineSchematic::parse(&result).unwrap().1;
    let schematic = Schematic::parse(&result).unwrap().1;
    // dbg!(schematic);

    // part 1
    let not_adjacent_sum = schematic
        .pieces
        .iter()
        .enumerate()
        .fold(0, |acc, (row_number, row)| {
            row.iter()
                .filter(|piece| match piece.part {
                    Part::Number(_) => true,
                    _ => false,
                })
                .fold(0, |acc, piece| {
                    let (previous_row, next_row) = match row_number {
                        0 => (None, Some(1)),
                        val => (
                            Some(val - 1),
                            if val >= schematic.pieces.len() {
                                None
                            } else {
                                Some(val + 1)
                            },
                        ),
                    };
                    if let Some(previous) = previous_row {
                        let row = schematic.pieces.get(previous).unwrap();
                        if row
                            .iter()
                            .filter(|piece_compare| match piece_compare.part {
                                Part::Symbol => true,
                                _ => false,
                            })
                            .any(|piece_compare| piece_compare.col_range.contains((piece.col_range.start - 1)..(piece.col_range.end+1)))
                    }
                    if let Some(next) = next_row {
                        let row = schematic.pieces.get(next).unwrap();
                    }
                    row.iter().any(|piece_before| todo!());
                    todo!()
                })
        });
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
}

impl Schematic<'_> {
    fn parse(input: &str) -> IResult<&str, Schematic> {
        let (input, pieces) = separated_list1(line_ending, Schematic::parse_pieces)(input)?;

        Ok((input, Schematic { pieces }))
    }

    fn parse_pieces(input: &str) -> IResult<&str, Vec<Piece>> {
        let (input, (parts, _)) = many_till(
            alt((
                map(digit1, |s: &str| Part::Number(s)),
                map(take(1usize), |s: &str| match s {
                    "." => Part::Period,
                    _ => Part::Symbol,
                }),
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

#[derive(Debug)]
struct EngineSchematic {
    matrix: Vec<Vec<char>>,
}

impl EngineSchematic {
    fn parse(input: &str) -> IResult<&str, EngineSchematic> {
        let (input, matrix) = separated_list1(line_ending, EngineSchematic::parse_row)(input)?;

        Ok((input, EngineSchematic { matrix }))
    }
    fn parse_row(input: &str) -> IResult<&str, Vec<char>> {
        let (input, (row, _)) = many_till(
            map(take(1usize), |s: &str| s.chars().next().unwrap()),
            peek(line_ending),
        )(input)?;

        Ok((input, row))
    }

    fn solve(&self) -> Vec<u32> {
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if let Some(char) = char.to_digit(10) {}
            }
        }

        todo!()
    }
}
