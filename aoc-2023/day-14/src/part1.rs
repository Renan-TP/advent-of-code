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
    fn get_loads(&self) -> usize {
        (0..self.max_x)
            .map(|i| {
                // dbg!(self.get_load_of_the_column(i));
                self.get_load_of_the_column(i)
            })
            .sum()
    }

    fn get_load_of_the_column(&self, column_number: usize) -> usize {
        let mut loads = 0usize;
        let rounds: Vec<usize> = self
            .round_rocks
            .iter()
            .filter(|(x, _)| *x == column_number)
            .map(|(_, y)| *y)
            .sorted()
            .collect();
        let cubes: Vec<usize> = self
            .cube_rocks
            .iter()
            .filter(|(x, _)| *x == column_number)
            .map(|(_, y)| *y)
            .sorted()
            .collect();
        let number_of_cubes = cubes.len();
        let number_of_rounds = rounds.len();

        if number_of_cubes == 0 {
            return self.calculate_load_before_cube(number_of_rounds);
        }

        if number_of_rounds == 0 {
            return 0;
        }

        if number_of_cubes == 1 {
            //Check how many before and after
            let before_rounds = rounds
                .into_iter()
                .filter(|round| round < cubes.first().expect("get only cube"))
                .count();
            let after_rounds = number_of_rounds - before_rounds;

            // dbg!((before_rounds, after_rounds));
            return self.calculate_load_before_cube(before_rounds)
                + self.calculate_load_after_cube(
                    after_rounds,
                    *cubes.first().expect("get only cube"),
                );
        }

        let before_rounds = rounds
            .clone()
            .into_iter()
            .filter(|round| round < cubes.first().expect("get only cube"))
            .count();
        let after_rounds = rounds
            .clone()
            .into_iter()
            .filter(|round| round > cubes.last().expect("get only cube"))
            .count();

        loads += self.calculate_load_before_cube(before_rounds);
        loads +=
            self.calculate_load_after_cube(after_rounds, *cubes.last().expect("get only cube"));
        // dbg!((&rounds, &cubes));

        let number_of_rounds_and_location = cubes
            .clone()
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                let number_of_rounds = rounds
                    .clone()
                    .into_iter()
                    .filter(|x| x > a && x < b)
                    .count();
                self.calculate_load_after_cube(number_of_rounds, *a)
            })
            .sum::<usize>();

        // dbg!(&number_of_rounds_and_location);

        loads += number_of_rounds_and_location;
        loads
    }
    fn calculate_load_before_cube(&self, number_of_rounds: usize) -> usize {
        if number_of_rounds == 0 {
            return 0;
        }
        number_of_rounds * (self.max_y + self.max_y - number_of_rounds + 1) / 2
    }
    fn calculate_load_after_cube(&self, number_of_rounds: usize, cube_position: usize) -> usize {
        if number_of_rounds == 0 {
            return 0;
        }
        let value_of_last_round = self.max_y - (cube_position + number_of_rounds);
        let value_of_start_round = self.max_y - (cube_position + 1);
        // dbg!((cube_position, value_of_start_round, value_of_last_round));
        number_of_rounds * (value_of_start_round + value_of_last_round) / 2
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
    // let res = dish_coordinate.get_load_of_the_column(2);
    dish_coordinate.get_loads().to_string()
    // let max_x = input.first().expect("get first line").len();
    // let max_y = input.len();
    // let mut after_tilt = vec![vec!['.'; max_x]; max_y];

    // todo!();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "O....#....
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
O....#....    OOOO.#.O.. 10 0
O.OO#....#    OO..#....#  9 1
.....##...    OO..O##..O  8 2
OO.#O....O    O..#.OO...  7 3
.O.....O#.    ........#.  6 4
O.#..O.#.#    ..#....#.#  5 5
..O..#O..O    ..O..#.O.O  4 6
.......O..    ..O.......  3 7
#....###..    #....###..  2 8
#OO..#....    #....#....  1 9

        34 27 17 10 8 7 7 14 0 12
*/
