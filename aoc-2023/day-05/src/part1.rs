use std::collections::HashSet;

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace0, multispace1, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Map<'a> {
    _entry: &'a str,
    _destination: &'a str,
    map: Vec<Vec<u64>>,
}
impl<'a> Map<'a> {
    fn next_link(&self, source_link: u64) -> u64 {
        let mut dest = source_link;
        let _ = self.map.iter().any(|link| {
            let (max_source, _) = (link[1] + link[2], link[0] + link[2]);
            let (min_source, min_dest) = (link[1], link[0]);
            if source_link >= min_source && source_link <= max_source {
                dest = min_dest + (source_link - min_source);
                // dbg!(&source_link, &link, &dest);
                return true;
            }
            false
        });

        dest
    }
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    // dbg!(input);
    let (input, (_entry, _destination)) = delimited(
        multispace0,
        separated_pair(alpha1, tag("-to-"), alpha1),
        tag(" map:"),
    )(input)?;
    // dbg!(input);
    let (input, rows) = delimited(
        multispace0,
        separated_list1(multispace1, separated_list1(space1, complete::u64)),
        line_ending,
    )(input)?;
    // dbg!(input);
    Ok((
        input,
        Map {
            _entry,
            _destination,
            map: rows,
        },
    ))
}
fn seed(input: &str) -> IResult<&str, HashSet<u64>> {
    let (input, seeds) = separated_list1(space1, complete::u64)(input)?;
    Ok((input, seeds.into_iter().collect()))
}
fn parse_input(input: &str) -> IResult<&str, u64> {
    let (input, seeds) = delimited(tag("seeds: "), seed, line_ending)(input)?;
    dbg!(input, &seeds);
    let (input, maps) = separated_list1(multispace1, parse_map)(input)?;
    // dbg!(input, maps);
    Ok((
        input,
        seeds.iter().fold(u64::MAX, |mut acc, seed| {
            let location = maps.iter().fold(*seed, |mut l_acc, map| {
                l_acc = map.next_link(l_acc);
                l_acc
            });
            dbg!(&location);
            acc = acc.min(location);
            acc
        }),
    ))
}

pub fn process(input: &str) -> String {
    // dbg!(parse(input));
    let (_, result) = parse_input(input).unwrap();
    result.to_string()
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
        assert_eq!("35", process(input))
    }
}
