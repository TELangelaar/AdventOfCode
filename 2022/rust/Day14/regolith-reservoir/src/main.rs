#![feature(iter_intersperse)]

use std::fmt::Display;

use nom::{
    bytes::complete::tag, character::complete::line_ending, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        )
    });

    let mut paths = paths(&result).unwrap().1;
    println!("{:?}", paths);

    // get lowest and highest x to make the grid
    let lowest_x = paths
        .iter()
        .map(|Path { data }| data.iter().map(|pos| pos.x).min().unwrap())
        .min()
        .unwrap();

    let highest_x = paths
        .iter()
        .map(|Path { data }| data.iter().map(|pos| pos.x).max().unwrap())
        .max()
        .unwrap();

    let highest_y = paths
        .iter()
        .map(|Path { data }| data.iter().map(|pos| pos.y).max().unwrap())
        .max()
        .unwrap();

    let offset_left = 300;
    let offset_right = 300;
    for path in paths.iter_mut() {
        for pos in path.data.iter_mut() {
            pos.x = pos.x - lowest_x + offset_left;
        }
    }

    println!("{:?}", paths);

    let sand_opening = Position {
        x: 500 - lowest_x + offset_left,
        y: 0,
    };
    let cols = (highest_x - lowest_x + offset_left + offset_right + 1) as usize;
    let rows = (highest_y + 1) as usize;
    let mut cave = Cave::new(sand_opening, cols, rows);

    cave.draw_paths(&paths);
    println!("{}", cave);
    let units_of_sand = cave.simulate();

    println!("\nUnits of sand: {}", units_of_sand);
}

#[derive(Debug)]
struct Cave {
    opening: Position,
    grid: Vec<Vec<char>>,
}

enum SimulationResult {
    Continue,
    Stop,
}

enum CaveResult {
    Blocked,
    NotBlocked,
    IntoTheVoid,
}

impl Cave {
    fn new(opening: Position, cols: usize, rows: usize) -> Self {
        let grid = vec![vec!['.'; cols]; rows + 2];
        Cave { opening, grid }
    }

    fn draw_paths(&mut self, paths: &Vec<Path>) {
        for path in paths {
            for window in path.data.windows(2) {
                let first = &window[0];
                let second = &window[1];

                let positions = get_positions_to_fill(first, second);
                for position in positions {
                    self.grid[position.y as usize][position.x as usize] = '#';
                }
            }
        }

        let last_y = self.grid.len() - 1;
        let last_x = self.grid[0].len() - 1;
        for idx in 0..=last_x {
            self.grid[last_y][idx] = '#';
        }
    }

    fn simulate(&mut self) -> u32 {
        let mut iterations = 0;
        let mut state = SimulationResult::Continue;
        while let SimulationResult::Continue = state {
            let sand_grain = self.opening.clone();
            iterations = iterations + 1;

            if self.grid[self.opening.y as usize][self.opening.x as usize] == 'O' {
                state = SimulationResult::Stop;
            } else {
                self.simulate_internal(sand_grain, &mut state);
            }

            // println!("\n== After iteration {iterations} ==");
            // println!("{}", self);
        }

        iterations - 1
    }

    fn simulate_internal(&mut self, mut sand_grain: Position, state: &mut SimulationResult) {
        self.fall_until_blocked(&mut sand_grain);
        match self.left_down_is_blocked(&sand_grain) {
            CaveResult::Blocked => match self.right_down_is_blocked(&sand_grain) {
                CaveResult::Blocked => {
                    self.grid[sand_grain.y as usize][sand_grain.x as usize] = 'O'
                }

                CaveResult::NotBlocked => {
                    sand_grain.x += 1;
                    sand_grain.y += 1;
                    self.simulate_internal(sand_grain, state)
                }
                CaveResult::IntoTheVoid => *state = SimulationResult::Stop,
            },
            CaveResult::NotBlocked => {
                sand_grain.x -= 1;
                sand_grain.y += 1;
                self.simulate_internal(sand_grain, state)
            }
            CaveResult::IntoTheVoid => *state = SimulationResult::Stop,
        }
    }

    fn fall_until_blocked(&self, sand_grain: &mut Position) -> CaveResult {
        let mut next = &self.grid[(sand_grain.y + 1) as usize][sand_grain.x as usize];
        while *next != '#' && *next != 'O' {
            sand_grain.y += 1;
            next = &self.grid[(sand_grain.y + 1) as usize][sand_grain.x as usize];
        }
        CaveResult::Blocked
    }

    fn left_down_is_blocked(&self, sand_grain: &Position) -> CaveResult {
        if sand_grain.x > 0 {
            if sand_grain.y < self.grid.len() as u32 {
                let next = &self.grid[(sand_grain.y + 1) as usize][(sand_grain.x - 1) as usize];
                if *next == '#' || *next == 'O' {
                    return CaveResult::Blocked;
                }
                return CaveResult::NotBlocked;
            }
        }
        CaveResult::IntoTheVoid
    }

    fn right_down_is_blocked(&self, sand_grain: &Position) -> CaveResult {
        if sand_grain.x < (self.grid[0].len() - 1) as u32 {
            if sand_grain.y < self.grid.len() as u32 {
                let next = &self.grid[(sand_grain.y + 1) as usize][(sand_grain.x + 1) as usize];
                if *next == '#' || *next == 'O' {
                    return CaveResult::Blocked;
                }
                return CaveResult::NotBlocked;
            }
        }
        CaveResult::IntoTheVoid
    }
}

fn get_positions_to_fill(first: &Position, second: &Position) -> Vec<Position> {
    //TODO: Refactor this
    let x_min: u32;
    let y_min: u32;
    let x_max: u32;
    let y_max: u32;

    if first.x == second.x {
        x_min = first.x;
        x_max = second.x;

        y_min = first.y.min(second.y);
        y_max = first.y.max(second.y);
    } else {
        y_min = first.y;
        y_max = second.y;

        x_min = first.x.min(second.x);
        x_max = first.x.max(second.x);
    }

    let mut positions = vec![];
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            positions.push(Position { x, y })
        }
    }

    positions
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|vec| vec.iter().collect::<String>())
                .intersperse("\n".to_string())
                .collect::<String>()
        )
    }
}

#[derive(Debug)]
struct Path {
    data: Vec<Position>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: u32,
    y: u32,
}

fn position(input: &str) -> IResult<&str, Position> {
    let (input, (x, y)) = separated_pair(
        nom::character::complete::u32,
        tag(","),
        nom::character::complete::u32,
    )(input)?;

    Ok((input, Position { x, y }))
}

fn path(input: &str) -> IResult<&str, Path> {
    let (input, data) = separated_list1(tag(" -> "), position)(input)?;

    Ok((input, Path { data }))
}

fn paths(input: &str) -> IResult<&str, Vec<Path>> {
    let (input, paths) = separated_list1(line_ending, path)(input)?;

    Ok((input, paths))
}
