use nom::{
    character::complete::{self, line_ending, multispace1},
    multi::separated_list1,
    IResult,
};

fn parse_block(input: &str) -> IResult<&str, u64> {
    let (input, calories) = separated_list1(line_ending, complete::u64)(input)?;
    // dbg!(&input);
    Ok((input, calories.iter().sum()))
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, calories) = separated_list1(multispace1, parse_block)(input)?;
    Ok((input, calories))
}

pub fn process(input: &str) -> String {
    let (_, calories) = parse(input).expect("should parse OK");
    calories
        .iter()
        .max()
        .expect("should return max calories value")
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!("24000", process(input))
    }
}
