use nom::{
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
#[derive(Debug, Clone, Copy)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug, Clone, Copy)]
struct Round {
    my_pick: Pick,
    other_pick: Pick,
}
impl Round {
    fn get_point(&self) -> u32 {
        match (self.my_pick, self.other_pick) {
            (Pick::Rock, Pick::Rock) => 1 + 3,
            (Pick::Rock, Pick::Paper) => 1 + 0,
            (Pick::Rock, Pick::Scissors) => 1 + 6,
            (Pick::Paper, Pick::Rock) => 2 + 6,
            (Pick::Paper, Pick::Paper) => 2 + 3,
            (Pick::Paper, Pick::Scissors) => 2 + 0,
            (Pick::Scissors, Pick::Rock) => 3 + 0,
            (Pick::Scissors, Pick::Paper) => 3 + 6,
            (Pick::Scissors, Pick::Scissors) => 3 + 3,
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, Round> {
    let (input, (other_pick, my_pick)) = separated_pair(alpha1, space1, alpha1)(input)?;
    let my_pick = match my_pick {
        "X" => Pick::Rock,
        "Y" => Pick::Paper,
        "Z" => Pick::Scissors,
        _ => panic!("Invalid input"),
    };
    let other_pick = match other_pick {
        "A" => Pick::Rock,
        "B" => Pick::Paper,
        "C" => Pick::Scissors,
        _ => panic!("Invalid input"),
    };
    Ok((
        input,
        Round {
            my_pick,
            other_pick,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Round>> {
    let (input, rounds) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, rounds))
}

pub fn process(input: &str) -> String {
    let (_, rounds) = parse(input).expect("Parse OK");
    // dbg!(&rounds);
    rounds
        .iter()
        .map(|round| round.get_point())
        .sum::<u32>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "A Y
B X
C Z";
        assert_eq!("15", process(input))
    }
}
