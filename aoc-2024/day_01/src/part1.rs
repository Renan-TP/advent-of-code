use nom::{self, character::complete::{multispace1, newline}, multi::separated_list1, sequence::tuple, IResult};

fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (first, _, second)) = tuple((
        nom::character::complete::u32,
        multispace1,
        nom::character::complete::u32,
    ))(input)?;

    Ok((input, (first, second)))
}
fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (remaining, lines) = separated_list1(newline, parse_line)(input)?;

    let (list1, list2): (Vec<u32>, Vec<u32>) = lines.into_iter().unzip();
    Ok((remaining, (list1, list2)))
}
pub fn process(input: &str) -> String {
    match parse_input(input) {
        Ok((_, (mut list1, mut list2))) => {
            // println!("list1: {:?}", list1);
            // println!("list2: {:?}", list2);
            list1.sort();
            list2.sort();
            return list1.iter().enumerate().map(|(n, value)| value.abs_diff(list2[n])).sum::<u32>().to_string();
        }
        Err(err) => {
            eprintln!("Error parsing input: {:?}", err);
        }
    }
    "0".to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input))
    }
}
