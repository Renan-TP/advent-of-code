use std::ops::Range;

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, multispace0, space0, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
#[derive(Debug)]
struct Map {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}
impl Map {
    fn navigation(&self, source: u64) -> u64 {
        0u64
    }
}
fn parse_map(input: &str) -> IResult<&str, Map> {
    // dbg!(input);
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(parse_line)).map(|mappings| Map { mappings }))
        .parse(input)
}
fn parse_line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    // dbg!(input);
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;
    // dbg!(input);
    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<Map>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(multispace0, complete::u64))
        .parse(input)?;
    let mut seed_range: Vec<(Range<u64>)> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        seed_range.push(seeds[i]..(seeds[i] + seeds[i + 1]));
    }
    // dbg!(&input, &seeds);
    let (input, seed_maps) = many1(parse_map)(input)?;
    // dbg!(input, maps);
    Ok((input, (seed_range, seed_maps)))
}

pub fn process(input: &str) -> String {
    // dbg!(parse(input));
    let (_, (seeds, maps)) = parse_input(input).expect("Should parse");
    dbg!(&seeds, &maps);
    // let location = seeds
    //     .iter()
    //     .map(|seed| maps.iter().fold(*seed, |seed, map| map.navigation(seed)))
    //     .collect::<Vec<u64>>();
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

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
56 93 4";
        assert_eq!("46", process(input))
    }
}
