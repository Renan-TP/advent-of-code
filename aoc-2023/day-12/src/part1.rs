#[derive(Debug)]
struct DataStruct<'a> {
    records: Vec<&'a str>,
    damged_springs: Vec<Vec<u32>>,
}
impl<'a> DataStruct<'a> {
    fn solve(&self, row: usize) {
        let damged = &self.damged_springs[row];
        let record = self.records[row].chars();
        loop {}
    }
}

struct UnknowSpring {}

fn parse(input: &str) -> DataStruct {
    input.lines().fold(
        DataStruct {
            records: Vec::new(),
            damged_springs: Vec::new(),
        },
        |mut acc, line| {
            let mut iter = line.split_whitespace();
            acc.records.push(iter.next().expect("records"));
            acc.damged_springs.push(
                iter.next()
                    .expect("damged_springs")
                    .split(',')
                    .map(|number| number.parse::<u32>().expect("parse number ok"))
                    .collect::<Vec<u32>>(),
            );
            acc
        },
    )
}
pub fn process(input: &str) -> String {
    let result = parse(input);
    dbg!(result);

    todo!()
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
        assert_eq!("21", process(input))
    }
}

/*
(0, 3), (3, 3) | 1, 1, 3
(0, 2), (0, 2), (2, 3) | 1, 1, 3
(2, 15) | 1, 3, 1, 6
 */
