use nom::{
    bytes::complete::is_a,
    character::complete::{line_ending, multispace1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct MirrorPattern<'a> {
    lines: Vec<&'a str>,
}

impl<'a> MirrorPattern<'a> {
    fn horizontal_reflects(&self) -> Option<(usize, usize)> {
        for i in 0..self.lines.len() {
            if i == 0 && self.lines[0] == self.lines[1] {
                return Some((i, i + 1));
            }
            if i == self.lines.len() - 1 {
                break;
            }

            if self.lines[i] == self.lines[i + 1] {
                let max_check_line = (self.lines.len() - 1 - i - 1).min(i);
                if (1..=max_check_line).all(|j| {
                    // dbg!(self.lines[i - j], self.lines[i + 1 + j]);
                    self.lines[i - j] == self.lines[i + 1 + j]
                }) {
                    return Some((i, i + 1));
                }
            }
        }
        None
    }
    fn horizontal_reflects_with_smudge(&self) -> Option<(usize, usize)> {
        for i in 0..self.lines.len() {
            if i == 0 && self.lines[0] != self.lines[1] {
                let line1_iter = self.lines[1].chars().collect::<Vec<char>>();
                if self.lines[0]
                    .chars()
                    .enumerate()
                    .filter(|(i, c)| *c != line1_iter[*i])
                    .count()
                    == 1
                {
                    return Some((0, 1));
                }
            }
            if i == self.lines.len() - 1 {
                break;
            }

            if self.lines[i] == self.lines[i + 1] {
                let max_check_line = (self.lines.len() - 1 - i - 1).min(i);
                let mut position = 0usize;
                if (1..=max_check_line)
                    .filter(|j| {
                        if self.lines[i - j] != self.lines[i + 1 + j] {
                            position = *j;
                            return true;
                        }
                        false
                    })
                    .count()
                    == 1
                {
                    // dbg!((i, i + 1));
                    return Some((i, i + 1));
                }
            } else if i != 0 {
                let line1_iter = self.lines[1].chars().collect::<Vec<char>>();
                if self.lines[0]
                    .chars()
                    .enumerate()
                    .filter(|(i, c)| *c != line1_iter[*i])
                    .count()
                    == 1
                {
                    let max_check_line = (self.lines.len() - 1 - i - 1).min(i);
                    if (1..=max_check_line).all(|j| {
                        // dbg!(self.lines[i - j], self.lines[i + 1 + j]);
                        self.lines[i - j] == self.lines[i + 1 + j]
                    }) {
                        // dbg!((i, i + 1));
                        return Some((i, i + 1));
                    }
                }
            }
        }
        None
    }
    fn vertical_reflects_with_smudge(&self) -> Option<(usize, usize)> {
        let rotate_input = self.rotate();
        // let rotate_input: Vec<&str> = rotate_input.iter().map(|s| s.as_str()).collect();
        for i in 0..rotate_input.len() {
            // if i == 0 && rotate_input[0] == rotate_input[1] {
            //     return None;
            // }
            if i == 0 && rotate_input[0] != rotate_input[1] {
                let line1_iter = rotate_input[1].chars().collect::<Vec<char>>();
                if rotate_input[0]
                    .chars()
                    .enumerate()
                    .filter(|(i, c)| *c != line1_iter[*i])
                    .count()
                    == 1
                {
                    return Some((0, 1));
                }
            }
            if i == rotate_input.len() - 1 {
                break;
            }

            if rotate_input[i] == rotate_input[i + 1] {
                let max_check_line = (rotate_input.len() - 1 - i - 1).min(i);
                let mut position = 0usize;
                if (1..=max_check_line)
                    .filter(|j| {
                        if rotate_input[i - j] != rotate_input[i + 1 + j] {
                            position = *j;
                            return true;
                        }
                        false
                    })
                    .count()
                    == 1
                {
                    // dbg!((i, i + 1));
                    return Some((i, i + 1));
                }
            } else if i != 0 {
                let max_check_line = (rotate_input.len() - 1 - i - 1).min(i);
                if (1..=max_check_line).all(|j| {
                    // dbg!(rotate_input[i - j], rotate_input[i + 1 + j]);
                    rotate_input[i - j] == rotate_input[i + 1 + j]
                }) {
                    // dbg!((i, i + 1));
                    return Some((i, i + 1));
                }
            }
        }
        None
    }
    fn vertical_reflects(&self) -> Option<(usize, usize)> {
        let rotate_input = self.rotate();
        // let rotate_input: Vec<&str> = rotate_input.iter().map(|s| s.as_str()).collect();
        for i in 0..rotate_input.len() {
            if i == 0 && rotate_input[0] == rotate_input[1] {
                return Some((i, i + 1));
            }
            if i == rotate_input.len() - 1 {
                break;
            }

            if rotate_input[i] == rotate_input[i + 1] {
                let max_check_line = (rotate_input.len() - 1 - i - 1).min(i);
                if (1..=max_check_line).all(|j| {
                    // dbg!(&rotate_input[i - j], &rotate_input[i + 1 + j]);
                    rotate_input[i - j] == rotate_input[i + 1 + j]
                }) {
                    return Some((i, i + 1));
                }
            }
        }
        None
    }

    fn rotate(&self) -> Vec<String> {
        let v: Vec<Vec<char>> = self.lines.iter().map(|s| s.chars().collect()).collect();
        let n = v.len();
        let m = v.first().expect("ok").len();
        let mut new_v: Vec<Vec<char>> = Vec::new();
        for j in 0..m {
            let mut temp = Vec::new();
            (0..n).for_each(|i| {
                temp.push(v[i][j]);
            });
            new_v.push(temp);
        }
        // for i in 0..n {
        //     v[i].reverse();
        // }
        let new_v = new_v
            .into_iter()
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>();
        // dbg!((&new_v, &self.lines));
        new_v
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
    let (_, data) = parse(input).expect("Parse input OK");
    // dbg!(&data);
    let res = data
        .into_iter()
        .map(|p| {
            (
                p.horizontal_reflects_with_smudge(),
                p.vertical_reflects_with_smudge(),
            )
        })
        .fold(0usize, |mut acc, (h_res, v_res)| {
            if h_res.is_some() {
                // dbg!(&h_res.expect("ok"));
                acc += h_res.expect("Ok").1 * 100;
            }
            if v_res.is_some() {
                // dbg!(&v_res);
                acc += v_res.expect("Ok").1;
            }
            acc
        });
    res.to_string()
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
        assert_eq!("400", process(input))
    }
}
