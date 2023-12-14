use nom::{
    bytes::complete::is_a,
    character::complete::{alpha1, line_ending, multispace0, multispace1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct MirrorPattern<'a> {
    lines: Vec<&'a str>,
}

impl<'a> MirrorPattern<'a> {
    fn horizontal_reflects(&self) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.lines.len() {
            if i == 0 && self.lines[0] == self.lines[1] {
                res.push((i, i + 1));
            }
            if i == self.lines.len() - 1 {
                break;
            }

            if self.lines[i] == self.lines[i + 1] {
                res.push((i, i + 1));
            }
        }
        res
    }
}
fn parse_pattern(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, is_a("#."))(input)
}
fn parse(input: &str) -> IResult<&str, Vec<MirrorPattern>> {
    let (input, data) = separated_list1(multispace1, parse_pattern)(input)?;

    Ok((
        input,
        data.into_iter()
            .map(|d| MirrorPattern { lines: d })
            .collect(),
    ))
}

pub fn process(input: &str) -> String {
    let (input, data) = parse(input).expect("Parse input OK");
    dbg!(&data);
    let res = data
        .into_iter()
        .map(|p| p.horizontal_reflects())
        .collect::<Vec<Vec<(usize, usize)>>>();
    dbg!(&res);
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        assert_eq!("405", process(input))
    }
}
