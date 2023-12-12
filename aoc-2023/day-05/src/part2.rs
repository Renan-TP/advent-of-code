use std::ops::Range;

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

#[derive(Debug)]
struct Map {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}
impl Map {
    // fn navigation_forward(&self, source: u64) -> u64 {
    //     let valid_mapping = self
    //         .mappings
    //         .iter()
    //         .find(|(source_range, _)| source_range.contains(&source));
    //     let Some((source_range, destination_range)) = valid_mapping else {
    //         return source;
    //     };
    //     let offset = source - source_range.start;
    //     destination_range.start + offset
    // }
    fn navigation_backward(&self, destination: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(_, destination_range)| destination_range.contains(&destination));
        let Some((source_range, destination_range)) = valid_mapping else {
            return destination;
        };
        let offset = destination - destination_range.start;
        source_range.start + offset
    }
    // fn smallest_destination_range(&self) -> Range<u64> {
    //     let (start, end) =
    //         self.mappings
    //             .iter()
    //             .fold((u64::MAX, 0), |(mut start, mut end), (_, d)| {
    //                 if start >= d.start {
    //                     start = d.start;
    //                     end = d.end;
    //                 }
    //                 // dbg!(&range);
    //                 (start, end)
    //             });
    //     start..end
    // }
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
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, space1, complete::u64)
                .map(|(start, end)| start..(start + end)),
        ))
        .parse(input)?;
    // dbg!(&input, &seeds);
    let (input, seed_maps) = many1(parse_map)(input)?;
    // dbg!(input, maps);
    Ok((input, (seeds, seed_maps)))
}

// fn forward_solution(seeds: Vec<Range<u64>>, maps: Vec<Map>) -> String {
//     let location = seeds
//         .par_iter()
//         .flat_map(|seed_range| seed_range.clone())
//         .collect::<Vec<u64>>();
//     let location = location
//         .into_iter()
//         .progress()
//         .map(|seed| {
//             maps.iter()
//                 .fold(seed, |seed, map| map.navigation_forward(seed))
//         })
//         .collect::<Vec<u64>>();

//     location.iter().min().expect("Get min value").to_string()
// }

fn backward_solution(seeds: Vec<Range<u64>>, maps: Vec<Map>) -> String {
    /*Third attemp*/
    let max_range_location = 0..u64::MAX;
    let result = max_range_location
        .into_iter()
        .find(|&location| {
            let s = maps
                .iter()
                .rev()
                .fold(location, |location, map| map.navigation_backward(location));
            seeds.iter().any(|seed_range| seed_range.contains(&s))
        })
        .expect("Get smallest location")
        .to_string();
    result

    /*Second attemp*/
    // let smallest_destination_range = maps
    //     .last()
    //     .expect("get last map")
    //     .smallest_destination_range();
    // // dbg!(smaller_destination_range);
    // let start_location = smallest_destination_range.start;
    // let seed_min_range = smallest_destination_range
    //     .map(|location| {
    //         maps.iter()
    //             .rev()
    //             .fold(location, |location, map| map.navigation_backward(location))
    //     })
    //     .collect::<Vec<u64>>();
    // let (id, source_seed) = seed_min_range
    //     .iter()
    //     .enumerate()
    //     .find(|(_, seed)| seeds.iter().any(|seed_range| seed_range.contains(&seed)))
    //     .expect("found smaller source seed");
    // dbg!(maps
    //     .iter()
    //     .fold(*source_seed, |ok_seed, map| map.navigation_forward(ok_seed))
    //     .to_string());
    // (start_location + (id as u64)).to_string()
}

pub fn process(input: &str) -> String {
    // dbg!(parse(input));
    let (_, (seeds, maps)) = parse_input(input).expect("Should parse");
    // forward_solution(seeds, maps)
    backward_solution(seeds, maps)
    // todo!()
    // dbg!(&seeds, &maps);
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
