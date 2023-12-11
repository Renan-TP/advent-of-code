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
enum Pipe {
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

fn map_pipe(c: &char) -> Result<Pipe, Pipe> {
    match c {
        '|' => Ok(Pipe::Vertical),
        '-' => Ok(Pipe::Horizontal),
        'L' => Ok(Pipe::NorthEastL),
        'J' => Ok(Pipe::NorthWestJ),
        '7' => Ok(Pipe::SouthWest7),
        'F' => Ok(Pipe::SouthEastF),
        '.' => Ok(Pipe::Ground),
        'S' => Ok(Pipe::StartPoint),
        _ => Err(Pipe::Ground),
    }
}

fn parser(input: &str) -> ((usize, usize), Vec<Vec<Pipe>>) {
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
    data: &Vec<Vec<Pipe>>,
) -> Vec<(Position, (usize, usize))> {
    let near_points = search_near_points(&(*x, *y), data);
    near_points
        .into_iter()
        .filter(|(p, (c, line))| is_connected(&data[*y][*x], p, &data[*line][*c]))
        .collect()
}
fn available_directions_exclude(
    (x, y): &(usize, usize),
    data: &Vec<Vec<Pipe>>,
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
fn is_connected(start: &Pipe, start_position: &Position, pipe: &Pipe) -> bool {
    match start {
        Pipe::Vertical => match start_position {
            Position::Top => matches!(pipe, Pipe::SouthWest7 | Pipe::Vertical | Pipe::SouthEastF),
            Position::Bottom => {
                matches!(pipe, Pipe::NorthWestJ | Pipe::Vertical | Pipe::NorthEastL)
            }
            _ => false,
        },
        Pipe::Horizontal => match start_position {
            Position::Left => {
                matches!(pipe, Pipe::Horizontal | Pipe::NorthEastL | Pipe::SouthEastF)
            }
            Position::Right => {
                matches!(pipe, Pipe::Horizontal | Pipe::SouthWest7 | Pipe::NorthWestJ)
            }
            _ => false,
        },
        Pipe::NorthEastL => match start_position {
            Position::Top => matches!(pipe, Pipe::SouthWest7 | Pipe::Vertical | Pipe::SouthEastF),
            Position::Right => {
                matches!(pipe, Pipe::Horizontal | Pipe::SouthWest7 | Pipe::NorthWestJ)
            }
            _ => false,
        },
        Pipe::NorthWestJ => match start_position {
            Position::Top => matches!(pipe, Pipe::SouthWest7 | Pipe::Vertical | Pipe::SouthEastF),
            Position::Left => {
                matches!(pipe, Pipe::Horizontal | Pipe::NorthEastL | Pipe::SouthEastF)
            }
            _ => false,
        },
        Pipe::SouthWest7 => match start_position {
            Position::Left => {
                matches!(pipe, Pipe::Horizontal | Pipe::NorthEastL | Pipe::SouthEastF)
            }
            Position::Bottom => {
                matches!(pipe, Pipe::NorthWestJ | Pipe::Vertical | Pipe::NorthEastL)
            }
            _ => false,
        },
        Pipe::SouthEastF => match start_position {
            Position::Right => {
                matches!(pipe, Pipe::Horizontal | Pipe::SouthWest7 | Pipe::NorthWestJ)
            }
            Position::Bottom => {
                matches!(pipe, Pipe::NorthWestJ | Pipe::Vertical | Pipe::NorthEastL)
            }
            _ => false,
        },
        Pipe::Ground => false,
        Pipe::StartPoint => match start_position {
            Position::Top => matches!(pipe, Pipe::SouthWest7 | Pipe::Vertical | Pipe::SouthEastF),
            Position::Left => {
                matches!(pipe, Pipe::Horizontal | Pipe::NorthEastL | Pipe::SouthEastF)
            }
            Position::Right => {
                matches!(pipe, Pipe::Horizontal | Pipe::SouthWest7 | Pipe::NorthWestJ)
            }
            Position::Bottom => {
                matches!(pipe, Pipe::NorthWestJ | Pipe::Vertical | Pipe::NorthEastL)
            }
        },
    }
}

fn search_near_points(
    (x, y): &(usize, usize),
    data: &Vec<Vec<Pipe>>,
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
    let ((x, y), data) = parser(input.trim());
    // dbg!((&x, &y), &data);
    let res = available_direction(&(x, y), &data);
    let mut step = 1u64;
    // dbg!(&res);
    let mut direction_1 = available_directions_exclude(&res[0].1, &data, res[0].0);
    let mut direction_2 = available_directions_exclude(&res[1].1, &data, res[1].0);
    step += 1;
    // dbg!((&direction_1, &direction_2));
    // dbg!((&direction_1, &direction_2));
    // dbg!(&res);
    loop {
        if direction_1.1 == direction_2.1 {
            return step.to_string();
        }
        // dbg!(&direction_1.1, &direction_2.1);
        direction_1 = available_directions_exclude(&direction_1.1, &data, direction_1.0);
        direction_2 = available_directions_exclude(&direction_2.1, &data, direction_2.0);
        step += 1;
    }
    // todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        assert_eq!("4", process(input))
    }
    #[test]
    fn test_process_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
        assert_eq!("8", process(input))
    }
}
