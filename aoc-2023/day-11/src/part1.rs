use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct DataStruct<'a> {
    image: Vec<&'a str>,
    galaxies: HashSet<Coordinate>,
    double_colunm: HashSet<usize>,
    double_row: HashSet<usize>,
}

impl<'a> DataStruct<'a> {
    fn get_pair_set(&self) -> HashSet<(&Coordinate, &Coordinate)> {
        let mut pairs = HashSet::new();
        for galaxy in &self.galaxies {
            for other_galaxy in &self.galaxies {
                if galaxy != other_galaxy {
                    if pairs.contains(&(other_galaxy, galaxy))
                        || pairs.contains(&(galaxy, other_galaxy))
                    {
                        // dbg!((&galaxy, &other_galaxy));
                    } else {
                        pairs.insert((galaxy, other_galaxy));
                    }
                } else {
                    // dbg!((&galaxy, &other_galaxy));
                }
            }
        }
        pairs
    }
    fn calculate_path(&self, (a, b): &(Coordinate, Coordinate)) -> usize {
        let mut acc = 0usize;
        let range_of_row_passed = ((a.y).min(b.y))..((a.y).max(b.y));
        let range_of_column_passed = ((a.x).min(b.x))..((a.x).max(b.x));
        let double_row_passed = self
            .double_row
            .iter()
            .filter(|&row| range_of_row_passed.clone().any(|r| r == *row))
            .count();
        let double_column_passed = self
            .double_colunm
            .iter()
            .filter(|&column| range_of_column_passed.clone().any(|r| r == *column))
            .count();
        // dbg!(&double_column_passed);
        // dbg!(
        //     (a, b),
        //     (
        //         &range_of_row_passed.clone().count(),
        //         &range_of_column_passed.clone().count()
        //     )
        // );

        acc += range_of_row_passed.count()
            + range_of_column_passed.count()
            + double_row_passed
            + double_column_passed;
        acc
        // todo!()
    }
}

fn parse(input: &str) -> DataStruct {
    let line_len = input.find('\n').expect("end line");
    input.lines().enumerate().fold(
        DataStruct {
            image: Vec::new(),
            galaxies: HashSet::new(),
            double_colunm: (0..line_len).collect(),
            double_row: HashSet::new(),
        },
        |mut acc_data, (line_index, line)| {
            acc_data.image.push(line.trim());
            if !line.chars().any(|c| c == '#') {
                acc_data.double_row.insert(line_index);
            }
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    acc_data.galaxies.insert(Coordinate { x, y: line_index });
                }
            });
            let list_of_row_blank = line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '.')
                .map(|(index, _)| index)
                .collect::<Vec<usize>>();
            acc_data
                .double_colunm
                .retain(|c| list_of_row_blank.contains(c));
            acc_data
        },
    )
}

pub fn process(input: &str) -> String {
    let data = parse(input);
    // dbg!(&data);
    let pairs = data.get_pair_set();
    // dbg!(pairs.iter().count());
    pairs
        .iter()
        .fold(0usize, |mut acc, &(a, b)| {
            acc += data.calculate_path(&(*a, *b));
            acc
        })
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        assert_eq!("374", process(input))
    }

    #[test]
    fn test_process_2() {
        let input = "...#......
.......#..
";
        assert_eq!("8", process(input))
    }
}
