use indicatif::ProgressIterator;
use num::Integer;
use std::collections::{HashMap, HashSet};
// use indicatif::ProgressIterator;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace1, space0},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_map(input: &str) -> IResult<&str, (&str, Node)> {
    let (input, (entry, (left, right))) = separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)?;
    Ok((input, (entry, Node { left, right })))
}

fn parse(input: &str) -> IResult<&str, (&str, &str, HashMap<&str, Node>)> {
    let (input, navigate) = delimited(space0, alpha1, multispace1)(input)?;
    let (input, map) = separated_list1(line_ending, parse_map)(input)?;
    Ok((
        input,
        (
            navigate,
            map.first().expect("get first ok").0,
            map.into_iter().collect(),
        ),
    ))
}
type FindWayResult<I, V> = Result<(I, V), (I, V)>;
fn find_way<'a>(
    navigate: &'a str,
    entry: &'a str,
    map: &'a HashMap<&'a str, Node>,
) -> FindWayResult<usize, Vec<&'a str>> {
    let way_to_go = navigate.chars().fold(vec![entry], |mut destination, c| {
        match c {
            'L' => destination.push(
                map.get(destination.last().expect("should have lasted destination"))
                    .expect("Should get to next node")
                    .left,
            ),
            'R' => destination.push(
                map.get(destination.last().expect("should have lasted destination"))
                    .expect("Should get to next node")
                    .right,
            ),
            _ => unreachable!(),
        }
        destination
    });
    // dbg!(&way_to_go);
    match way_to_go
        .iter()
        .enumerate()
        .find(|(_, &dest)| dest.ends_with('Z'))
    {
        Some((step, _)) => Ok((step, way_to_go)),
        None => Err((navigate.len(), way_to_go)),
    }
}
pub fn process(input: &str) -> String {
    let (_, (navigate, _, map)) = parse(input).expect("should paser ok");
    let parallel_entry = map
        .clone()
        .into_iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(entry, _)| entry)
        .collect::<Vec<&str>>();
    dbg!(&parallel_entry);
    parallel_entry
        .into_iter()
        .progress()
        .map(|entry| {
            let mut start_entry = entry;
            let mut total_step = 0usize;
            loop {
                match find_way(navigate, start_entry, &map) {
                    Ok((step, _)) => {
                        total_step += step;

                        return total_step as i64;
                    }
                    Err((step, destinations)) => {
                        start_entry = destinations.last().expect("get last location");
                        total_step += step;
                    }
                }
            }
        })
        .fold(1i64, |mut lcm, z_node| {
            dbg!(&z_node);
            lcm = lcm.lcm(&z_node);
            lcm
        })
        .to_string()
    // dbg!(result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "LR

CCA = (CCB, XXX)
CCB = (XXX, CCZ)
CCZ = (CCB, XXX)
DDA = (DDB, XXX)
DDB = (DDC, DDC)
DDC = (DDZ, DDZ)
DDZ = (DDB, DDB)
XXX = (XXX, XXX)
";
        assert_eq!("6", process(input))
    }
}
