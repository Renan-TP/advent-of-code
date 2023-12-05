use std::collections::HashSet;

#[derive(Debug)]
struct Schematic<'a> {
    width: usize,
    height: usize,
    string_lines: Vec<&'a str>,
    matrix: Vec<Vec<char>>,
}
/*
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.. */
impl<'a> Schematic<'a> {
    fn find_valid_and_sum(&self) -> u32 {
        let mut symbol_position: HashSet<(usize, usize)> = HashSet::new();
        self.matrix.iter().enumerate().for_each(|(r, row)| {
            row.iter().enumerate().for_each(|(c, ch)| {
                if !ch.is_ascii_digit() && !'.'.eq(ch) {
                    check_rc(self, r, c).iter().for_each(|&position| {
                        symbol_position.insert(position);
                    });
                }
            });
        });
        symbol_position
            .iter()
            .map(|(r, c)| -> u32 {
                let mut cc = *c;
                while cc + 1 < self.width && self.matrix[*r][cc + 1].is_ascii_digit() {
                    cc += 1;
                }
                // dbg!((r, c), &self.string_lines[*r][*c..=cc]);
                self.string_lines[*r][*c..=cc]
                    .parse::<u32>()
                    .expect("Should change to nunber")
            })
            .sum()
    }
}

fn check_cc(schematic: &Schematic, cr: usize, c: usize) -> HashSet<usize> {
    let mut out: HashSet<usize> = HashSet::new();
    if c > 0 && c < schematic.width {
        for cc in [c - 1, c, c + 1] {
            if !schematic.matrix[cr][cc].is_ascii_digit() {
                continue;
            }
            let mut cc_out: usize = cc;
            while cc_out > 0 && schematic.matrix[cr][cc_out - 1].is_ascii_digit() {
                cc_out -= 1;
            }
            out.insert(cc_out);
        }
    } else if c == 0 {
        for cc in [c, c + 1] {
            if !schematic.matrix[cr][cc].is_ascii_digit() {
                continue;
            }
            let mut cc_out: usize = cc;
            while cc_out > 0 && schematic.matrix[cr][cc_out - 1].is_ascii_digit() {
                cc_out -= 1;
            }
            out.insert(cc_out);
        }
    } else {
        for cc in [c - 1, c] {
            if !schematic.matrix[cr][cc].is_ascii_digit() {
                continue;
            }
            let mut cc_out: usize = cc;
            while cc_out > 0 && schematic.matrix[cr][cc_out - 1].is_ascii_digit() {
                cc_out -= 1;
            }
            out.insert(cc_out);
        }
    }
    out
}

fn check_rc(schematic: &Schematic, r: usize, c: usize) -> HashSet<(usize, usize)> {
    let mut out = HashSet::new();
    if r > 0 && r < schematic.height {
        for cr in [r - 1, r, r + 1] {
            check_cc(schematic, cr, c).iter().for_each(|&cc_out| {
                out.insert((cr, cc_out));
            });
        }
    } else if r == 0 {
        for cr in [r, r + 1] {
            check_cc(schematic, cr, c).iter().for_each(|&cc_out| {
                out.insert((cr, cc_out));
            });
        }
    } else {
        for cr in [r - 1, r] {
            check_cc(schematic, cr, c).iter().for_each(|&cc_out| {
                out.insert((cr, cc_out));
            });
        }
    }
    out
}

pub fn process(input: &str) -> String {
    let schematic: Schematic = Schematic {
        width: input
            .lines()
            .collect::<Vec<&str>>()
            .first()
            .expect("get the first line")
            .len(),
        height: input.lines().count(),
        string_lines: input.lines().collect(),
        matrix: input.lines().fold(Vec::new(), |mut acc, row| {
            acc.push(row.chars().collect::<Vec<char>>());
            acc
        }),
    };
    dbg!(&schematic.string_lines);
    schematic.find_valid_and_sum().to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input))
    }
    //520701
    //535078
}
