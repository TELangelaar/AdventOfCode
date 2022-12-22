use nom::{
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        )
    });

    let heatmap = heatmap(&result).unwrap().1;
    println!("{heatmap:?}");

    for i in 0..heatmap.len() {
        for j in 0..heatmap[i].len() {
            if heatmap[i][j] == Elevation::Start {
                let current = Position { row: i, col: j };
                let mut paths = vec![];
                if current.row > 0 {
                    let next = Position {
                        row: current.row - 1,
                        col: current.col,
                    };
                    paths.push(next);
                }
                if current.col < heatmap[current.row].len() - 1 {
                    let next = Position {
                        row: current.row,
                        col: current.col + 1,
                    };
                    paths.push(next);
                }
                if current.row < heatmap.len() - 1 {
                    let next = Position {
                        row: current.row + 1,
                        col: current.col,
                    };
                    paths.push(next);
                }
                if current.col > 0 {
                    let next = Position {
                        row: current.row,
                        col: current.col - 1,
                    };
                    paths.push(next);
                }
                let mut least_steps = 10_000;
                for next in paths {
                    let result = solve_part1(&current, &next, &heatmap);
                    match result {
                        SolveResult::End(steps) => {
                            if steps < least_steps {
                                least_steps = steps;
                            }
                        }
                        SolveResult::DeadEnd => (),
                    }
                }
                println!("Least steps: {least_steps}");
            }
        }
    }
}

fn solve_part1(current: &Position, next: &Position, heatmap: &Vec<Vec<Elevation>>) -> SolveResult {
    let current_elevation = &heatmap[current.row][current.col];
    match current_elevation {
        Elevation::Start => SolveResult::End(get_least_steps(next, current, heatmap)),
        Elevation::End => panic!("This shouldnt happen [End]"),
        Elevation::Height(own_height) => {
            let next_elevation = &heatmap[next.row][next.col];
            match next_elevation {
                Elevation::Start => SolveResult::DeadEnd,
                Elevation::End => {
                    let x = ('x' as u8) - 96;
                    if *own_height > x {
                        SolveResult::End(1)
                    } else {
                        SolveResult::DeadEnd
                    }
                }
                Elevation::Height(other_height) => {
                    let range = other_height - 1..=other_height + 1;
                    if range.contains(own_height) {
                        SolveResult::End(get_least_steps(next, current, heatmap))
                    } else {
                        SolveResult::DeadEnd
                    }
                }
            }
        }
    }
}

enum SolveResult {
    End(u32),
    DeadEnd,
}

fn get_least_steps(current: &Position, previous: &Position, heatmap: &Vec<Vec<Elevation>>) -> u32 {
    println!("Entered get_least_steps with:");
    println!("\tCurrent {current:?}");
    println!("\tPrevious {previous:?}");
    let mut paths = vec![];
    if current.row > 0 {
        let next = Position {
            row: current.row - 1,
            col: current.col,
        };
        if next != *previous {
            let result = solve_part1(current, &next, heatmap);
            paths.push(result);
        }
    }

    if current.col < heatmap[current.row].len() - 1 {
        let next = Position {
            row: current.row,
            col: current.col + 1,
        };
        if next != *previous {
            let result = solve_part1(current, &next, heatmap);
            paths.push(result);
        }
    }

    if current.row < heatmap.len() - 1 {
        let next = Position {
            row: current.row + 1,
            col: current.col,
        };
        if next != *previous {
            let result = solve_part1(current, &next, heatmap);
            paths.push(result);
        }
    }

    if current.col > 0 {
        let next = Position {
            row: current.row,
            col: current.col - 1,
        };
        if next != *previous {
            let result = solve_part1(current, &next, heatmap);
            paths.push(result);
        }
    }

    let mut least_steps = 10_000;
    for path in paths {
        match path {
            SolveResult::End(steps) => {
                let steps = steps + 1;
                if steps < least_steps {
                    least_steps = steps;
                }
            }
            SolveResult::DeadEnd => (),
        };
    }
    println!("Exited get_least_steps with: {least_steps}");
    least_steps
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Elevation {
    Start,
    End,
    Height(u8),
}

fn elevation(input: &str) -> IResult<&str, Vec<Elevation>> {
    let (input, parsed) = alpha1(input)?;
    let mut result = vec![];
    for c in parsed.chars() {
        let c = match c {
            'S' => Elevation::Start,
            'E' => Elevation::End,
            char => {
                let height = (char as u8) - 96;
                Elevation::Height(height)
            }
        };
        result.push(c);
    }

    Ok((input, result))
}

fn heatmap(input: &str) -> IResult<&str, Vec<Vec<Elevation>>> {
    let (input, heatmap) = separated_list1(line_ending, elevation)(input)?;

    Ok((input, heatmap))
}
