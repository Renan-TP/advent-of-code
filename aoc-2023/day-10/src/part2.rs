use std::collections::HashSet;

/*
   | is a vertical pipe connecting north and south.
   - is a horizontal pipe connecting east and west.
   L is a 90-degree bend connecting north and east.
   J is a 90-degree bend connecting north and west.
   7 is a 90-degree bend connecting south and west.
   F is a 90-degree bend connecting south and east.
   . is ground; there is no pipe in this tile.
   S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/
#[derive(Debug, Clone, Copy)]
enum Tile {
    Vertical = '|' as isize,
    Horizontal = '-' as isize,
    NorthEastL = 'L' as isize,
    NorthWestJ = 'J' as isize,
    SouthWest7 = '7' as isize,
    SouthEastF = 'F' as isize,
    Ground = '.' as isize,
    StartPoint = 'S' as isize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Position {
    // TopLeft,
    Top,
    // TopRight,
    Left,
    Right,
    // BottomLeft,
    Bottom,
    // BottomRight,
}
// #[derive(Debug, Clone, Copy)]
// enum Direction {
//     WestToEastRL,
//     EastToWestLR,
//     NorthToSouthUD,
//     SouthToNorthDU,
// }

fn map_pipe(c: &char) -> Result<Tile, Tile> {
    match c {
        '|' => Ok(Tile::Vertical),
        '-' => Ok(Tile::Horizontal),
        'L' => Ok(Tile::NorthEastL),
        'J' => Ok(Tile::NorthWestJ),
        '7' => Ok(Tile::SouthWest7),
        'F' => Ok(Tile::SouthEastF),
        '.' => Ok(Tile::Ground),
        'S' => Ok(Tile::StartPoint),
        _ => Err(Tile::Ground),
    }
}

fn parser(input: &str) -> ((usize, usize), Vec<Vec<Tile>>) {
    let data = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>());
    let data_result = data
        .clone()
        .map(|line| {
            line.iter()
                .map(|c| map_pipe(c).expect("should parse to enum"))
                .collect()
        })
        .collect();
    let (line_index, chars) = data
        .enumerate()
        .find(|(_, line)| line.iter().any(|c| *c == 'S'))
        .expect("found S");
    let (char_index, _) = chars
        .iter()
        .enumerate()
        .find(|(_, &c)| c == 'S')
        .expect("found S");

    ((char_index, line_index), data_result)
}

/*
        [7|F]
    [L-F]S[7-J]
        [J|L]
*/
fn available_direction(
    (x, y): &(usize, usize),
    data: &Vec<Vec<Tile>>,
) -> Vec<(Position, (usize, usize))> {
    let near_points = search_near_points(&(*x, *y), data);
    near_points
        .into_iter()
        .filter(|(p, (c, line))| is_connected(&data[*y][*x], p, &data[*line][*c]))
        .collect()
}
fn available_directions_exclude(
    (x, y): &(usize, usize),
    data: &Vec<Vec<Tile>>,
    position_compare_to_source: Position,
) -> (Position, (usize, usize)) {
    let exculde_direction = match position_compare_to_source {
        Position::Top => Position::Bottom,
        Position::Left => Position::Right,
        Position::Right => Position::Left,
        Position::Bottom => Position::Top,
    };
    let near_points = search_near_points(&(*x, *y), data)
        .into_iter()
        .collect::<Vec<(Position, (usize, usize))>>();
    // dbg!(&near_points);
    let connected_pipes = near_points
        .into_iter()
        .filter(|(p, (c, line))| is_connected(&data[*y][*x], p, &data[*line][*c]))
        .filter(|(p, _)| exculde_direction != *p)
        .collect::<Vec<(Position, (usize, usize))>>();
    connected_pipes[0]
}
/*
        [7|F]
    [L-F]S[7-J]
        [J|L]
*/
fn is_connected(start: &Tile, start_position: &Position, pipe: &Tile) -> bool {
    match start {
        Tile::Vertical => match start_position {
            Position::Top => matches!(pipe, Tile::SouthWest7 | Tile::Vertical | Tile::SouthEastF),
            Position::Bottom => {
                matches!(pipe, Tile::NorthWestJ | Tile::Vertical | Tile::NorthEastL)
            }
            _ => false,
        },
        Tile::Horizontal => match start_position {
            Position::Left => {
                matches!(pipe, Tile::Horizontal | Tile::NorthEastL | Tile::SouthEastF)
            }
            Position::Right => {
                matches!(pipe, Tile::Horizontal | Tile::SouthWest7 | Tile::NorthWestJ)
            }
            _ => false,
        },
        Tile::NorthEastL => match start_position {
            Position::Top => matches!(pipe, Tile::SouthWest7 | Tile::Vertical | Tile::SouthEastF),
            Position::Right => {
                matches!(pipe, Tile::Horizontal | Tile::SouthWest7 | Tile::NorthWestJ)
            }
            _ => false,
        },
        Tile::NorthWestJ => match start_position {
            Position::Top => matches!(pipe, Tile::SouthWest7 | Tile::Vertical | Tile::SouthEastF),
            Position::Left => {
                matches!(pipe, Tile::Horizontal | Tile::NorthEastL | Tile::SouthEastF)
            }
            _ => false,
        },
        Tile::SouthWest7 => match start_position {
            Position::Left => {
                matches!(pipe, Tile::Horizontal | Tile::NorthEastL | Tile::SouthEastF)
            }
            Position::Bottom => {
                matches!(pipe, Tile::NorthWestJ | Tile::Vertical | Tile::NorthEastL)
            }
            _ => false,
        },
        Tile::SouthEastF => match start_position {
            Position::Right => {
                matches!(pipe, Tile::Horizontal | Tile::SouthWest7 | Tile::NorthWestJ)
            }
            Position::Bottom => {
                matches!(pipe, Tile::NorthWestJ | Tile::Vertical | Tile::NorthEastL)
            }
            _ => false,
        },
        Tile::Ground => false,
        Tile::StartPoint => match start_position {
            Position::Top => matches!(pipe, Tile::SouthWest7 | Tile::Vertical | Tile::SouthEastF),
            Position::Left => {
                matches!(pipe, Tile::Horizontal | Tile::NorthEastL | Tile::SouthEastF)
            }
            Position::Right => {
                matches!(pipe, Tile::Horizontal | Tile::SouthWest7 | Tile::NorthWestJ)
            }
            Position::Bottom => {
                matches!(pipe, Tile::NorthWestJ | Tile::Vertical | Tile::NorthEastL)
            }
        },
    }
}

fn search_near_points(
    (x, y): &(usize, usize),
    data: &Vec<Vec<Tile>>,
) -> Vec<(Position, (usize, usize))> {
    let max_x = data.last().expect("get last").len() - 1;
    let max_y = data.len() - 1;
    // dbg!((max_x, max_y));
    if *x == 0 && *y == 0 {
        return vec![
            (Position::Right, (1, 0)),
            (Position::Bottom, (0, 1)),
            // (Position::BottomRight, (1, 1)),
        ];
    } else if *x == max_x && *y == max_y {
        return vec![
            // (Position::TopLeft, (max_x - 1, max_y - 1)),
            (Position::Left, (max_x - 1, max_y)),
            (Position::Top, (max_x, max_y - 1)),
        ];
    } else if *x == 0 && *y == max_y {
        return vec![
            (Position::Top, (0, max_y - 1)),
            // (Position::TopRight, (1, max_y - 1)),
            (Position::Right, (1, max_y)),
        ];
    } else if *x == max_x && *y == 0 {
        return vec![
            (Position::Left, (max_x - 1, 0)),
            // (Position::BottomLeft, (max_x - 1, 1)),
            (Position::Bottom, (max_x, 1)),
        ];
    } else if *x == 0 {
        return vec![
            (Position::Top, (0, y - 1)),
            (Position::Bottom, (0, y + 1)),
            // (Position::TopRight, (1, y - 1)),
            (Position::Right, (1, *y)),
            // (Position::BottomRight, (1, y + 1)),
        ];
    } else if *y == 0 {
        return vec![
            (Position::Left, (x - 1, 0)),
            // (Position::BottomLeft, (x - 1, 1)),
            (Position::Bottom, (*x, 1)),
            (Position::Right, (x + 1, 0)),
            // (Position::BottomRight, (x + 1, 1)),
        ];
    } else if *x == max_x {
        return vec![
            (Position::Top, (*x, y - 1)),
            (Position::Left, (x - 1, *y)),
            (Position::Bottom, (*x, y + 1)),
        ];
    } else if *y == max_y {
        return vec![
            (Position::Left, (x - 1, *y)),
            (Position::Top, (*x, y - 1)),
            (Position::Right, (x + 1, *y)),
        ];
    } else {
        return vec![
            // (Position::TopLeft, (x - 1, y - 1)),
            (Position::Left, (x - 1, *y)),
            // (Position::BottomLeft, (x - 1, y + 1)),
            (Position::Top, (*x, y - 1)),
            (Position::Bottom, (*x, y + 1)),
            // (Position::TopRight, (x + 1, y - 1)),
            (Position::Right, (x + 1, *y)),
            // (Position::BottomRight, (x + 1, y + 1)),
        ];
    }
}

pub fn process(input: &str) -> String {
    let ((x_start, y_start), data) = parser(input.trim());
    // dbg!((&x, &y), &data);
    let mut the_loop = HashSet::new();
    the_loop.insert((x_start, y_start));

    let res = available_direction(&(x_start, y_start), &data);
    the_loop.insert(res[0].1);
    the_loop.insert(res[1].1);
    // let mut step = 1u64;
    // dbg!(&res);
    let mut direction_1 = available_directions_exclude(&res[0].1, &data, res[0].0);
    let mut direction_2 = available_directions_exclude(&res[1].1, &data, res[1].0);
    // step += 1;
    the_loop.insert(direction_1.1);
    the_loop.insert(direction_2.1);
    // dbg!((&direction_1, &direction_2));
    // dbg!((&direction_1, &direction_2));
    // dbg!(&res);
    loop {
        if direction_1.1 == direction_2.1 {
            // the_loop.pop().expect("pop Ok");
            // dbg!(&the_loop);
            break;
        }
        // dbg!(&direction_1.1, &direction_2.1);
        direction_1 = available_directions_exclude(&direction_1.1, &data, direction_1.0);
        direction_2 = available_directions_exclude(&direction_2.1, &data, direction_2.0);
        the_loop.insert(direction_1.1);
        the_loop.insert(direction_2.1);
        // step += 1;
    }
    // dbg!(&the_loop);
    let mut is_inside = false;
    let mut inside_tile = 0u32;
    data.clone().iter().enumerate().for_each(|(y, line)| {
        is_inside = false;
        // dbg!(y);
        line.iter().enumerate().for_each(|(x, _)| {
            if the_loop
                .iter()
                .any(|(pipe_x, pipe_y)| x == *pipe_x && y == *pipe_y)
            {
                match data[y][x] {
                    Tile::Vertical | Tile::NorthWestJ | Tile::NorthEastL => {
                        is_inside = !is_inside;
                        // dbg!((x, y, is_inside));
                    }
                    Tile::StartPoint => {
                        if res.iter().any(|(p, _)| matches!(p, Position::Top)) {
                            is_inside = !is_inside;
                        }
                    }
                    _ => {
                        // is_inside = !is_inside;
                    }
                }
            } else if is_inside {
                // dbg!((x, y));
                inside_tile += 1;
            }
        });
        // dbg!(&inside_tile);
    });
    inside_tile.to_string()
    // todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        assert_eq!("4", process(input))
    }

    #[test]
    fn test_process_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        assert_eq!("8", process(input))
    }

    #[test]
    fn test_process_3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        assert_eq!("10", process(input))
    }
}
