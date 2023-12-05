use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};
#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
    // point: u32,
}
impl Card {
    fn get_point(&self) -> u32 {
        let power = self
            .winning_numbers
            .intersection(&self.player_numbers)
            .count() as u32;
        if power == 0u32 {
            return 0u32;
        }
        2u32.pow(power - 1)
    }
}
fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<u32>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}
fn cards(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        complete::u32,
        tuple((tag(":"), space1)),
    )(input)?;

    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winning_numbers, player_numbers)| Card {
            winning_numbers,
            player_numbers,
            // point: 0u32,
        })
        .parse(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, cards)(input)
}

pub fn process(input: &str) -> String {
    let (_input, cards) = parse_cards(input).expect("parse cards");
    dbg!(_input, &cards);
    cards
        .iter()
        .map(|card| card.get_point())
        .sum::<u32>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input))
    }
}
