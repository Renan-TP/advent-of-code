use nom::{
    character::complete::{self, multispace0, multispace1, space1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use nom_supreme::tag::complete::tag;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}
impl Race {
    fn formula(&self, button_time: u32) -> u32 {
        button_time * (self.time - button_time)
    }
    fn get_win_case(&self) -> Vec<u32> {
        (1..self.time)
            .rev()
            .filter(|&time| self.formula(time) > self.distance)
            .collect()
    }
    fn get_win_case_with_math(&self) -> u32 {
        let discriminant = self.time.pow(2) - (4 * self.distance);
        let range = ((self.time as f32 - (discriminant as f32).sqrt()) / 2f32)
            ..((self.time as f32 + (discriminant as f32).sqrt()) / 2f32);
        // dbg!(
        //     &range,
        //     (range.start.floor() as u32 + 1)..=(range.end.ceil() as u32 - 1)
        // );
        ((range.start.floor() as u32 + 1)..=(range.end.ceil() as u32 - 1)).count() as u32
    }
}
fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, result) = delimited(
        multispace1,
        separated_list1(space1, complete::u32),
        multispace0,
    )(input)?;
    Ok((input, result))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, time) = preceded(tag("Time:"), parse_line)(input)?;
    let (input, distance) = preceded(tag("Distance:"), parse_line)(input)?;
    // dbg!(time, distance);
    let result = time
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, &time)| {
            acc.push(Race {
                time,
                distance: distance[i],
            });
            acc
        });
    Ok((input, result))
}

pub fn process(input: &str) -> String {
    let (_, races) = parse_input(input).expect("parse success");
    // dbg!(&races);

    //Whitout math
    // races
    //     .iter()
    //     .fold(1u32, |mut acc, race| {
    //         acc *= race.get_win_case().len() as u32;
    //         acc
    //     })
    //     .to_string()

    races
        .iter()
        .fold(1u32, |mut acc, race| {
            acc *= race.get_win_case_with_math();
            acc
        })
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200
        ";
        assert_eq!("288", process(input))
    }
}
