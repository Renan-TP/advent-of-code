use std::collections::HashSet;

use nom::{
    character::complete::{alpha1, line_ending},
    multi::many1,
    sequence::delimited,
    IResult,
};
use nom_supreme::tag::complete::tag;

#[derive(Debug)]
struct Group<'a> {
    rucksacks: [&'a str; 3],
}
impl<'a> Group<'a> {
    fn compare(&self) -> char {
        if let Some(result) = self.rucksacks[0].chars().find(|fc| {
            self.rucksacks[1]
                .chars()
                .filter(|sc| self.rucksacks[2].chars().any(|tc| tc == *sc))
                .any(|after_fil| after_fil == *fc)
        }) {
            return result;
        }
        '\0'
    }

    fn look_up(&self, c: char, lookup_set: &HashSet<(char, u32)>) -> u32 {
        let (_, result) = lookup_set.iter().find(|(ch, _)| c == *ch).expect("found");
        *result
    }
}

fn parse_group(input: &str) -> IResult<&str, Group> {
    let (input, first) = delimited(tag(""), alpha1, line_ending)(input)?;
    let (input, second) = delimited(tag(""), alpha1, line_ending)(input)?;
    let (input, last) = delimited(tag(""), alpha1, line_ending)(input)?;
    Ok((
        input,
        Group {
            rucksacks: [first, second, last],
        },
    ))
    // let (input, first) = many1;
    // let (input, second) = delimited(tag(""), alpha1, line_ending)(input)?;
    // let (input, last) = delimited(tag(""), alpha1, line_ending)(input)?;
}

fn parse(input: &str) -> IResult<&str, Vec<Group>> {
    let (input, groups) = many1(parse_group)(input)?;
    Ok((input, groups))
}

pub fn process(input: &str) -> String {
    let (_, groups) = parse(input).expect("parse ok");
    // Map the letters to numbers
    let letters = 'a'..='z';
    let upper_letters = 'A'..='Z';
    let numbers = 1..=26;
    let upper_numbers = 27..=52;
    let mut lookup_set = letters.zip(numbers).collect::<HashSet<(char, u32)>>();
    lookup_set.extend(
        upper_letters
            .zip(upper_numbers)
            .collect::<HashSet<(char, u32)>>(),
    );
    groups
        .iter()
        .map(|group| group.look_up(group.compare(), &lookup_set))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!("70", process(input))
    }
}
