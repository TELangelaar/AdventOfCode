use std::{collections::HashMap, iter::zip, ops::Range, time::Instant};

use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::{many1, many_till, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
        )
    });

    let almanac = Almanac::parse(&result).unwrap().1;
    let locations = almanac.to_locations();
    println!(
        "Lowest location number: {:#?}",
        locations.iter().min().unwrap()
    );
}

#[derive(Debug)]
struct Seeds {
    values: Vec<Range<usize>>,
}

#[derive(Debug)]
struct Mapping {
    differences: Vec<isize>,
    source_ranges: Vec<Range<usize>>,
    dest_ranges: Vec<Range<usize>>
}

#[derive(Debug)]
struct Almanac {
    seeds: Seeds,
    mappings: Vec<Mapping>,
}

impl Mapping {
    fn parse_mapping(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
        let (input, _) = many1(line_ending)(input)?;
        let (input, _) = many_till(take(1usize), line_ending)(input)?;

        let map = tuple((
            map_res(digit1, |s: &str| s.parse::<usize>()),
            tag(" "),
            map_res(digit1, |s: &str| s.parse::<usize>()),
            tag(" "),
            map_res(digit1, |s: &str| s.parse::<usize>()),
        ));
        let (input, mappings) = separated_list1(line_ending, map)(input)?;

        Ok((
            input,
            mappings
                .into_iter()
                .map(|(dest, _, source, _, range)| (dest, source, range))
                .collect(),
        ))
    }

    fn create_map(mappings: Vec<(usize, usize, usize)>) -> Mapping {
        let mut differences = vec![];
        let mut source_ranges = vec![];
        let mut dest_ranges = vec![];
        for (dest, source, range) in mappings {
            differences.push(source as isize - dest as isize);
            source_ranges.push(source..(source + range));
            dest_ranges.push(dest..(dest+range));
        }

        Mapping {
            differences,
            source_ranges,
            dest_ranges
        }
    }
}

impl Seeds {
    fn parse(input: &str) -> IResult<&str, Seeds> {
        let (input, pairs) = preceded(
            tag("seeds:"),
            many1(preceded(
                tag(" "),
                separated_pair(
                    map_res(digit1, |s: &str| s.parse::<usize>()),
                    tag(" "),
                    map_res(digit1, |s: &str| s.parse::<usize>()),
                ),
            )),
        )(input)?;

        let seeds = Seeds {
            values: pairs
                .into_iter()
                .map(|(start, length)| start..(start + length))
                .collect(),
        };

        Ok((input, seeds))
    }
}

impl Almanac {
    fn to_locations(&self) -> Vec<usize> {
        // let mut locations = vec![];
        // for seed_range in &self.seeds.values {
        //     for seed in seed_range.start..seed_range.end {
        //         let mut location = seed;
        //         for mapping in &self.mappings {
        //             if let Some((index, _)) = mapping.source_ranges.iter().enumerate().find(|(_, s)| s.contains(&location)) {
        //                 let difference = *mapping.differences.get(index).unwrap();
        //                 location = ((location as isize) - difference) as usize;
        //             }
        //         }
        //         locations.push(location);
        //     }
        // }
        let mut sorted_locations = &self.mappings.last().unwrap().dest_ranges;
        sorted_locations.sort_unstable_by(|first, second| {
            first.start.partial_cmp(&second.start).unwrap()
        });

        let reversed_mappings = self.mappings.iter().rev().collect::<Vec<&Mapping>>();
        for location_range in sorted_locations {
            for humidity in
                ((location_range.start as isize) - diff)..((location_range.end as isize) - diff)
            {
                let mut seed = 0usize;
                for (idx, &mapping) in reversed_mappings.iter().enumerate() {
                    if idx == 0 {
                        continue;
                    }
                    // if let Some((index, _)) = mapping.source_ranges.iter().enumerate().find(|(_, s)| {((s.start as isize) - )}) {

                    // }
                }
            }
        }

        todo!()
    }

    fn parse(input: &str) -> IResult<&str, Almanac> {
        let (input, seeds) = Seeds::parse(input)?;

        let (input, seed_to_soil) = map(Mapping::parse_mapping, |mappings| {
            Mapping::create_map(mappings)
        })(input)?;
        let (input, soil_to_fertilizer) = map(Mapping::parse_mapping, |mappings| {
            Mapping::create_map(mappings)
        })(input)?;
        let (input, fertilizer_to_water) = map(Mapping::parse_mapping, |mappings| {
            Mapping::create_map(mappings)
        })(input)?;
        let (input, water_to_light) = map(Mapping::parse_mapping, |mappings| {
            Mapping::create_map(mappings)
        })(input)?;
        let (input, light_to_temperature) = map(Mapping::parse_mapping, |mappings| {
            Mapping::create_map(mappings)
        })(input)?;
        let (input, temperature_to_humidity) = map(Mapping::parse_mapping, |mappings| {
            Mapping::create_map(mappings)
        })(input)?;
        let (input, humidity_to_location) = map(Mapping::parse_mapping, |mappings| {
            Mapping::create_map(mappings)
        })(input)?;

        let mappings = vec![
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ];

        Ok((input, Almanac { seeds, mappings }))
    }
}
