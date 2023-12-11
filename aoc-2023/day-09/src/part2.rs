use itertools::{Itertools, Position};
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

fn parse_history(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, data) = separated_list1(space1, complete::i64)(input)?;
    Ok((input, data.into_iter().rev().collect()))
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, parse_history)(input)
}

pub fn process(input: &str) -> String {
    let (_, histories) = parse(input).expect("parse ok");
    histories
        .into_iter()
        .map(|h| {
            let mut nums = h;
            let mut end_numbers: Vec<i64> = vec![];
            loop {
                if nums.iter().all(|n| n == &0) {
                    break;
                }
                // dbg!(&nums);
                nums = nums
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        // dbg!((left, right));
                        match position {
                            Position::Last | Position::Only => {
                                end_numbers.push(*right);
                            }
                            _ => {}
                        };
                        right - left
                    })
                    .collect();
            }
            // dbg!(&end_numbers);
            end_numbers.iter().sum::<i64>()
        })
        .sum::<i64>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        assert_eq!("2", process(input))
    }
}
