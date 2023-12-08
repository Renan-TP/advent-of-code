use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace1, space0},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
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
        .find(|(_, &dest)| dest == "ZZZ")
    {
        Some((step, _)) => Ok((step, way_to_go)),
        None => Err((way_to_go.len(), way_to_go)),
    }
}

pub fn process(input: &str) -> String {
    let (_, (navigate, _, map)) = parse(input).expect("should paser ok");
    let mut total_step = 0usize;
    let mut entry = "AAA";
    loop {
        match find_way(navigate, entry, &map) {
            Ok((step, _)) => {
                total_step += step;

                return total_step.to_string();
            }
            Err((step, destinations)) => {
                entry = destinations.last().expect("get last location");
                total_step += step - 1;
            }
        }
    }
    // dbg!(result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input))
    }
    #[test]
    fn test_process_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input))
    }
}
