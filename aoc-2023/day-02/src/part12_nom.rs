use std::{collections::BTreeMap, ops::Not};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}
#[derive(Debug)]
struct Game<'a> {
    id: u32,
    rounds: Vec<Vec<Cube<'a>>>,
}
impl<'a> Game<'a> {
    fn valid_for_cube_set(&self, map: &BTreeMap<&str, u32>) -> Option<u32> {
        self.rounds
            .iter()
            .any(|round| {
                round.iter().any(|shown_cube| {
                    shown_cube.amount > *map.get(shown_cube.color).expect("a valid cube")
                })
            })
            .not()
            .then_some(self.id)
    }
    fn get_power_of_minimum_cube_set(&self) -> u32 {
        let map: BTreeMap<&str, u32> = BTreeMap::new();
        self.rounds
            .iter()
            .fold(map, |mut acc, round| {
                for cube in round.iter() {
                    acc.entry(cube.color)
                        .and_modify(|v| *v = (*v).max(cube.amount))
                        .or_insert(cube.amount);
                }
                acc
            })
            .values()
            .product()
    }
}
//4 red
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, space1, alpha1)(input)?;
    Ok((input, Cube { color, amount }))
}
//3 blue, 4 red
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}
//Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { id, rounds }))
}
//Next game
fn parse_game(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input).unwrap();
    dbg!(games.len());
    Ok((input, games))
}

pub fn process(input: &str) -> String {
    // dbg!(input);
    let games = parse_game(input).expect("Should parse");
    // dbg!(&games);
    let map = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    games
        .1
        .iter()
        .filter_map(|game| game.valid_for_cube_set(&map))
        .sum::<u32>()
        .to_string()
}
pub fn process_2(input: &str) -> String {
    let games = parse_game(input).expect("Should parse");
    games
        .1
        .iter()
        .map(|game| game.get_power_of_minimum_cube_set())
        .sum::<u32>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(input);
        assert_eq!(result, "8".to_string())
    }

    #[test]
    fn it_works_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process_2(input);
        assert_eq!(result, "2286".to_string())
    }
}
