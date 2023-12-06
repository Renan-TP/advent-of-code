use nom::{
    character::complete::{self, digit1, multispace0, multispace1, space1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use nom_supreme::tag::complete::tag;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    fn formula(&self, button_time: u64) -> u64 {
        button_time * (self.time - button_time)
    }
    fn get_win_case(&self) -> Vec<u64> {
        (1..self.time)
            .rev()
            .filter(|&time| self.formula(time) > self.distance)
            .collect()
    }
    fn get_win_case_with_math(&self) -> u64 {
        let discriminant = self.time.pow(2) - (4 * self.distance);
        (discriminant as f64).sqrt() as u64
    }
}
fn parse_line(input: &str) -> IResult<&str, u64> {
    let (input, result) =
        delimited(multispace1, separated_list1(space1, digit1), multispace0)(input)?;
    let result = result.join("");
    Ok((input, result.parse::<u64>().expect("should parse")))
}

fn parse_input(input: &str) -> IResult<&str, Race> {
    let (input, time) = preceded(tag("Time:"), parse_line)(input)?;
    let (input, distance) = preceded(tag("Distance:"), parse_line)(input)?;
    // dbg!(time, distance);

    Ok((input, Race { time, distance }))
}

pub fn process(input: &str) -> String {
    let (_, races) = parse_input(input).expect("parse success");
    // dbg!(&races);
    // races.get_win_case().len().to_string()
    races.get_win_case_with_math().to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200
        ";
        assert_eq!("71503", process(input))
    }
}
