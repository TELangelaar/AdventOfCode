#![feature(iter_intersperse)]
use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let args = &vec!["inputs.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        )
    });

    let mut pairs = pairs(&result).unwrap().1;
    println!("{:?}", pairs);

    let mut tunnels = Tunnels::new(10, &mut pairs);
    // println!("before drawing:\n{}", tunnels);
    for sbp in pairs {
        tunnels.draw(&sbp.sensor);
    }

    let answer = tunnels.count();

    // println!("{}", tunnels);
    println!("{}", answer);
}

#[derive(Debug)]
struct Tunnels {
    grid: Vec<char>,
    tracking_row: i32,
}

impl Tunnels {
    fn new(mut tracking_row: i32, pairs: &mut Vec<SensorBeaconPair>) -> Self {
        let min = get_min_x_y(&pairs);
        let max = get_max_x_y(&pairs);

        let offset_rows = 1_000_000;
        let offset_cols = 1_000_000;

        for i in 0..pairs.len() {
            let mut sbp = pairs[i].clone();
            sbp.normalize(&min);
            sbp.add_padding(offset_rows, offset_cols);
            sbp.calculate_distance();

            pairs[i] = sbp;
        }
        tracking_row = tracking_row + offset_rows - min.y;

        let cols = (max.x - min.x + offset_cols * 2 + 1) as usize;
        let mut grid = vec!['.'; cols];

        let mut max2 = (0, 0);
        let mut min2 = (0, 0);
        for (idx, sbp) in pairs.iter().enumerate() {
            if sbp.sensor.position.x > max2.0 {
                max2 = (sbp.sensor.position.x, idx);
            } else if sbp.sensor.position.x < min2.0 {
                min2 = (sbp.sensor.position.x, idx);
            }
            if sbp.beacon.position.x > max2.0 {
                max2 = (sbp.beacon.position.x, idx);
            } else if sbp.beacon.position.x < max2.0 {
                min2 = (sbp.beacon.position.x, idx);
            }
        }
        println!("Max sbp: {:?}", pairs[max2.1]);
        println!("Min sbp: {:?}", pairs[min2.1]);
        println!("Tracking row: {}", tracking_row);

        for sbp in pairs {
            if sbp.sensor.position.y == tracking_row {
                grid[sbp.sensor.position.x as usize] = sbp.sensor.tag;
            }
            if sbp.beacon.position.y == tracking_row {
                grid[sbp.beacon.position.x as usize] = sbp.beacon.tag;
            }
        }

        Tunnels { grid, tracking_row }
    }

    fn draw(&mut self, sbp: &Sensor) {
        match sbp.distance {
            Some(distance) => {
                let y_range =
                    (sbp.position.y as u32 - distance)..=(sbp.position.y as u32 + distance);

                if !y_range.contains(&(self.tracking_row as u32)) {
                    return;
                }

                let y_diff = (self.tracking_row - sbp.position.y).abs() as u32;
                let remaining_distance = distance - y_diff;

                for x in (sbp.position.x as u32 - remaining_distance)
                    ..=(sbp.position.x as u32 + remaining_distance)
                {
                    if self.grid[x as usize] != 'B' {
                        self.grid[x as usize] = '#';
                    }
                }
            }
            None => panic!("no distance!"),
        }
    }

    fn count(&self) -> u32 {
        let mut counter = 0;
        for ch in &self.grid {
            if *ch == '#' {
                counter += 1;
            }
        }
        counter
    }
}

impl Display for Tunnels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid.iter().collect::<String>())
    }
}

fn get_max_x_y(pairs: &[SensorBeaconPair]) -> Position {
    let mut max = Position { x: 0, y: 0 };
    for pair in pairs {
        let mut max_x = 0;
        let mut max_y = 0;
        if pair.sensor.position.x > pair.beacon.position.x {
            max_x = pair.sensor.position.x;
        } else {
            max_x = pair.beacon.position.x;
        }
        if pair.sensor.position.y > pair.beacon.position.y {
            max_y = pair.sensor.position.y;
        } else {
            max_y = pair.beacon.position.y;
        }

        if max_x > max.x {
            max.x = max_x;
        }
        if max_y > max.y {
            max.y = max_y;
        }
    }

    max
}

fn get_min_x_y(pairs: &[SensorBeaconPair]) -> Position {
    let mut min = Position { x: 0, y: 0 };
    for pair in pairs {
        let mut min_x = 0;
        let mut min_y = 0;
        if pair.sensor.position.x < pair.beacon.position.x {
            min_x = pair.sensor.position.x;
        } else {
            min_x = pair.beacon.position.x;
        }
        if pair.sensor.position.y < pair.beacon.position.y {
            min_y = pair.sensor.position.y;
        } else {
            min_y = pair.beacon.position.y;
        }

        if min_x < min.x {
            min.x = min_x;
        }
        if min_y < min.y {
            min.y = min_y;
        }
    }

    min
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

trait Adjust {
    fn add_padding(&mut self, offset_rows: i32, offset_cols: i32);
    fn normalize(&mut self, min: &Position);
}

#[derive(Debug, Clone)]
struct Sensor {
    position: Position,
    tag: char,
    distance: Option<u32>,
}

impl Adjust for Sensor {
    fn add_padding(&mut self, offset_rows: i32, offset_cols: i32) {
        self.position.x += offset_cols;
        self.position.y += offset_rows;
    }

    fn normalize(&mut self, min: &Position) {
        self.position.x -= min.x;
        self.position.y -= min.y;
    }
}

#[derive(Debug, Clone)]
struct Beacon {
    position: Position,
    tag: char,
}

impl Adjust for Beacon {
    fn add_padding(&mut self, offset_rows: i32, offset_cols: i32) {
        self.position.x += offset_cols;
        self.position.y += offset_rows;
    }

    fn normalize(&mut self, min: &Position) {
        self.position.x -= min.x;
        self.position.y -= min.y;
    }
}

#[derive(Debug, Clone)]
struct SensorBeaconPair {
    sensor: Sensor,
    beacon: Beacon,
}

impl SensorBeaconPair {
    fn calculate_distance(&mut self) {
        if self.sensor.position.x < 0
            || self.sensor.position.y < 0
            || self.beacon.position.x < 0
            || self.beacon.position.y < 0
        {
            panic!(
                "Normalize first! sensor: {:?}, beacon: {:?}",
                self.sensor, self.beacon
            );
        }

        if let Some(_) = self.sensor.distance {
            panic!("Already have a distance!");
        }

        let distance = ((self.sensor.position.x - self.beacon.position.x).abs()
            + (self.sensor.position.y - self.beacon.position.y).abs())
            as u32;
        self.sensor.distance = Some(distance);
    }
}

impl Adjust for SensorBeaconPair {
    fn add_padding(&mut self, offset_rows: i32, offset_cols: i32) {
        self.sensor.add_padding(offset_rows, offset_cols);
        self.beacon.add_padding(offset_rows, offset_cols);
    }

    fn normalize(&mut self, min: &Position) {
        self.sensor.normalize(min);
        self.beacon.normalize(min);
    }
}

fn position(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(
        tag("x="),
        separated_pair(
            nom::character::complete::i32,
            tag(", y="),
            nom::character::complete::i32,
        ),
    )(input)
}

fn pair(input: &str) -> IResult<&str, SensorBeaconPair> {
    let (input, (sensor, beacon)) = separated_pair(
        position.map(|(x, y)| Sensor {
            position: Position { x, y },
            tag: 'S',
            distance: None,
        }),
        tag(": closest beacon is at "),
        position.map(|(x, y)| Beacon {
            position: Position { x, y },
            tag: 'B',
        }),
    )(input)?;

    Ok((input, SensorBeaconPair { sensor, beacon }))
}

fn pairs(input: &str) -> IResult<&str, Vec<SensorBeaconPair>> {
    separated_list1(line_ending, preceded(tag("Sensor at "), pair))(input)
}
