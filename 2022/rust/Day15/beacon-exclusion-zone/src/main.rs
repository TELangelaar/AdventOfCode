#![feature(iter_intersperse)]

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

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

    let sensors = calculate_distances(&mut pairs);

    for sbp in pairs {
        let result = solve(&sbp.sensor, &sensors);
        if let Some(pos) = result {
            println!("Found the beacon at X: {}, Y: {}", pos.x, pos.y);
            println!(
                "{} * 4_000_000 + {} = {}",
                pos.x,
                pos.y,
                pos.x * 4_000_000 + pos.y
            );
            println!("");
            break;
        }
    }
}

fn calculate_distances(pairs: &mut Vec<SensorBeaconPair>) -> Vec<Sensor> {
    let mut perimeters = vec![];
    for i in 0..pairs.len() {
        let mut sbp = pairs[i].clone();
        sbp.calculate_distance();
        perimeters.push(sbp.sensor.clone());
        pairs[i] = sbp;
    }
    perimeters
}

fn solve(sensor: &Sensor, sensors: &Vec<Sensor>) -> Option<Position> {
    const MIN: i64 = 0;
    const MAX: i64 = 4_000_001;
    match sensor.distance {
        Some(distance) => {
            //walk around the perimeter
            let mut x_pos = (sensor.position.x, sensor.position.x);
            for y in (sensor.position.y - distance - 1)..=(sensor.position.y + distance + 1) {
                if y < MIN || y > MAX {
                    continue;
                }

                for x in [x_pos.0, x_pos.1] {
                    if x < MIN || x > MAX {
                        continue;
                    }
                    let pos = Position { x, y };
                    if sensors.not_within_distance(&pos) {
                        return Some(pos);
                    }
                }
                x_pos.0 -= 1;
                x_pos.1 += 1;
            }
            None
        }
        None => panic!("no distance!"),
    }
}

trait Manhattan {
    fn not_within_distance(&self, position: &Position) -> bool;
}

impl Manhattan for Vec<Sensor> {
    fn not_within_distance(&self, position: &Position) -> bool {
        self.iter().all(|sensor| {
            sensor.distance.unwrap()
                < ((sensor.position.x - position.x).abs() + (sensor.position.y - position.y).abs())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

trait Adjust {
    fn add_padding(&mut self, offset_rows: i64, offset_cols: i64);
    fn normalize(&mut self, min: &Position);
}

#[derive(Debug, Clone)]
struct Sensor {
    position: Position,
    distance: Option<i64>,
}

impl Adjust for Sensor {
    fn add_padding(&mut self, offset_rows: i64, offset_cols: i64) {
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
}

impl Adjust for Beacon {
    fn add_padding(&mut self, offset_rows: i64, offset_cols: i64) {
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
        if let Some(_) = self.sensor.distance {
            panic!("Already have a distance!");
        }

        let distance = (self.sensor.position.x - self.beacon.position.x).abs()
            + (self.sensor.position.y - self.beacon.position.y).abs();
        self.sensor.distance = Some(distance);
    }
}

impl Adjust for SensorBeaconPair {
    fn add_padding(&mut self, offset_rows: i64, offset_cols: i64) {
        self.sensor.add_padding(offset_rows, offset_cols);
        self.beacon.add_padding(offset_rows, offset_cols);
    }

    fn normalize(&mut self, min: &Position) {
        self.sensor.normalize(min);
        self.beacon.normalize(min);
    }
}

fn position(input: &str) -> IResult<&str, (i64, i64)> {
    preceded(
        tag("x="),
        separated_pair(
            nom::character::complete::i64,
            tag(", y="),
            nom::character::complete::i64,
        ),
    )(input)
}

fn pair(input: &str) -> IResult<&str, SensorBeaconPair> {
    let (input, (sensor, beacon)) = separated_pair(
        position.map(|(x, y)| Sensor {
            position: Position { x, y },
            distance: None,
        }),
        tag(": closest beacon is at "),
        position.map(|(x, y)| Beacon {
            position: Position { x, y },
        }),
    )(input)?;

    Ok((input, SensorBeaconPair { sensor, beacon }))
}

fn pairs(input: &str) -> IResult<&str, Vec<SensorBeaconPair>> {
    separated_list1(line_ending, preceded(tag("Sensor at "), pair))(input)
}
