use itertools::{repeat_n, Itertools};

#[derive(Debug)]
struct DataStruct {
    records: Vec<Record>,
}
#[derive(Debug)]
struct Record {
    line: String,
    batches: Vec<u32>,
    unknow_spring: u32,
}

impl Record {
    fn generate_combination(&self) -> impl Iterator<Item = Vec<char>> {
        repeat_n(['.', '#'].into_iter(), self.unknow_spring as usize).multi_cartesian_product()
    }
    fn check_combination(&self, combination: &Vec<char>) -> bool {
        let mut combination_iter = combination.iter();
        let fill_record = self
            .line
            .chars()
            .map(|c| match c {
                '?' => *combination_iter
                    .next()
                    .expect("should have enough for replacement"),
                value => value,
            })
            .group_by(|c| c == &'#')
            .into_iter()
            .filter_map(|(is_hash_group, group)| {
                is_hash_group.then_some(group.into_iter().count() as u32)
            })
            .collect::<Vec<u32>>();
        // dbg!(&fill_record);
        self.batches[..] == fill_record[..]
    }
    fn possible_solution_count(&self) -> u32 {
        self.generate_combination()
            .filter(|combination| self.check_combination(combination))
            .count() as u32
    }
}

impl DataStruct {
    fn solve(&self) -> String {
        self.records
            .iter()
            .map(|record| record.possible_solution_count())
            .sum::<u32>()
            .to_string()
    }
}

fn parse(input: &str) -> DataStruct {
    input.lines().fold(
        DataStruct {
            records: Vec::new(),
        },
        |mut acc, line| {
            let mut iter = line.split_whitespace();
            let data = iter.next().expect("records");
            let unknow_spring = data.chars().filter(|c| c == &'?').count() as u32;
            let unknow_spring = unknow_spring * 5 + 4;
            let data = std::iter::repeat(data).take(5).join("?");
            let batches = iter
                .next()
                .expect("damged_springs")
                .split(',')
                .map(|number| number.parse::<u32>().expect("parse number ok"))
                .collect::<Vec<u32>>();
            let batches = std::iter::repeat(batches).take(5).flatten().collect_vec();
            let record = Record {
                line: data,
                batches,
                unknow_spring,
            };
            acc.records.push(record);
            acc
        },
    )
}
pub fn process(input: &str) -> String {
    let data = parse(input);
    data.solve()
    // dbg!(result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        assert_eq!("525152", process(input))
    }
}

/*
(0, 3), (3, 3) | 1, 1, 3
(0, 2), (0, 2), (2, 3) | 1, 1, 3
(2, 15) | 1, 3, 1, 6
 */
