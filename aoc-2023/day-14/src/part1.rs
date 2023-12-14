use itertools::Itertools;

#[derive(Debug)]
enum RockType {
    Empty,
    Round,
    Cube,
}
#[derive(Debug)]
struct Coordinate {
    round_rocks: Vec<(usize, usize)>,
    cube_rocks: Vec<(usize, usize)>,
    max_x: usize,
    max_y: usize,
}

impl Coordinate {
    fn get_load_of_the_column(&self, column_number: usize) -> usize {
        let rounds: Vec<usize> = self
            .round_rocks
            .iter()
            .filter(|(x, y)| *x == column_number)
            .map(|(_, y)| *y)
            .sorted()
            .collect();
        let cubes: Vec<usize> = self
            .cube_rocks
            .iter()
            .filter(|(x, y)| *x == column_number)
            .map(|(_, y)| *y)
            .sorted()
            .collect();

        dbg!((&rounds, &cubes));

        let count = cubes
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                (
                    rounds
                        .clone()
                        .into_iter()
                        .filter(|x| x > a && x < b)
                        .count(),
                    *a,
                )
            })
            .collect::<Vec<(usize, usize)>>();
        dbg!(&count);
        let first_cube = cubes.first().expect("first cube");
        let last_cube = cubes.last().expect("last cube");
        let heavy_rounds = rounds
            .clone()
            .into_iter()
            .filter(|r| r < first_cube)
            .count();
        let light_rounds = rounds.into_iter().filter(|r| r > last_cube).count();

        let heavy_rounds = self.calculate_load(self.max_y - heavy_rounds + 1, self.max_y);

        let light_rounds = self.calculate_load(
            self.max_y - last_cube + 1,
            self.max_y - last_cube + light_rounds,
        );
        dbg!((&heavy_rounds, &light_rounds));
        todo!()
    }
    fn calculate_load(&self, n: usize, m: usize) -> usize {
        (m - n + 1) * (m + n) / 2
    }
    fn calculate_load_on_coordinate(&self, number_of_rounds: usize, coordinate: usize) -> usize {
        let load_at_coordinate = self.max_y - coordinate + 1;
    }
}

fn parse(input: &str) -> Vec<Vec<RockType>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => RockType::Cube,
                    'O' => RockType::Round,
                    _ => RockType::Empty,
                })
                .collect::<Vec<RockType>>()
        })
        .collect()
}

pub fn process(input: &str) -> String {
    let input = parse(input);
    let max_x = input.first().expect("get first line").len();
    let max_y = input.len();
    // dbg!(&input);
    let dish_coordinate = input.iter().enumerate().fold(
        Coordinate {
            round_rocks: Vec::new(),
            cube_rocks: Vec::new(),
            max_x,
            max_y,
        },
        |mut acc, (y, line)| {
            line.iter().enumerate().for_each(|(x, rock)| match rock {
                RockType::Round => acc.round_rocks.push((x, y)),
                RockType::Cube => acc.cube_rocks.push((x, y)),
                _ => {}
            });
            acc
        },
    );
    // dbg!(&dish_coordinate);
    let res = dish_coordinate.get_load_of_the_column(2);
    // let max_x = input.first().expect("get first line").len();
    // let max_y = input.len();
    // let mut after_tilt = vec![vec!['.'; max_x]; max_y];

    todo!();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        assert_eq!("136", process(input))
    }
}
/*
OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1
*/
