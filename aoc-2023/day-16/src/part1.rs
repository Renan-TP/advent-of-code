use glam::IVec2;
use std::{
    collections::{HashMap, HashSet},
    io::empty,
};

#[derive(Debug)]
enum MirrorType {
    Forwardslash,
    Backslash,
    Vertical,
    Horizontal,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
#[derive(Debug)]
struct Beam {
    location: IVec2,
    direction: Direction,
}

pub fn process(input: &str) -> String {
    let borders = (
        input
            .lines()
            .last()
            .expect("get last line should be ok")
            .chars()
            .count() as i32,
        input.lines().count() as i32,
    );
    let contraption =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut contraption, (y, line)| {
                line.chars().enumerate().for_each(|(x, char)| match char {
                    '/' => {
                        contraption
                            .insert(IVec2::new(x as i32, y as i32), MirrorType::Forwardslash);
                    }
                    '\\' => {
                        contraption.insert(IVec2::new(x as i32, y as i32), MirrorType::Backslash);
                    }
                    '|' => {
                        contraption.insert(IVec2::new(x as i32, y as i32), MirrorType::Vertical);
                    }
                    '-' => {
                        contraption.insert(IVec2::new(x as i32, y as i32), MirrorType::Horizontal);
                    }
                    _ => (),
                });
                contraption
            });
    // dbg!(&contraption);

    let mut energized: HashMap<IVec2, Vec<Direction>> = HashMap::new();
    energized.insert(IVec2::new(0, 0), vec![Direction::Right]);

    let mut beams = vec![Beam {
        location: IVec2 { x: 0, y: 0 },
        direction: Direction::Right,
    }];
    while let Some(mut beam) = beams.pop() {
        loop {
            if beam.location.cmplt(IVec2::new(0, 0)).any()
                || beam.location.cmpge(IVec2::new(borders.0, borders.1)).any()
            {
                break;
            }

            if let Some(mirror) = contraption.get(&beam.location) {
                dbg!(&mirror);
                next_move(&mut beam, &mirror, &mut energized);
            } else {
                match beam.direction {
                    Direction::Right => beam.location.x += 1,
                    Direction::Left => beam.location.x -= 1,
                    Direction::Up => beam.location.y -= 1,
                    Direction::Down => beam.location.y += 1,
                }
            }
        }
    }
    // dbg!(&energized);

    for y in 0..borders.1 {
        for x in 0..borders.0 {
            if energized.contains_key(&IVec2::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    energized.len().to_string()
}
fn energized_process(
    beam: &mut Beam,
    mirror: &MirrorType,
    energized: &mut HashMap<IVec2, Vec<Direction>>,
) -> bool {
    if let Some(energized_direction) = energized.get_mut(&beam.location) {
        return match mirror {
            MirrorType::Forwardslash => match beam.direction {
                Direction::Right | Direction::Down => energized_direction
                    .iter()
                    .any(|d| d == &Direction::Right || d == &Direction::Down),
                Direction::Left | Direction::Up => energized_direction
                    .iter()
                    .any(|d| d == &Direction::Left || d == &Direction::Up),
            },
            MirrorType::Backslash => match beam.direction {
                Direction::Left | Direction::Down => energized_direction
                    .iter()
                    .any(|d| d == &Direction::Left || d == &Direction::Down),
                Direction::Right | Direction::Up => energized_direction
                    .iter()
                    .any(|d| d == &Direction::Right || d == &Direction::Up),
            },
            MirrorType::Vertical => match beam.direction {
                Direction::Right | Direction::Left => {
                    energized_direction
                        .iter()
                        .any(|d| d == &Direction::Right || d == &Direction::Left)
                        || (energized_direction.contains(&Direction::Up)
                            && energized_direction.contains(&Direction::Down))
                }
                Direction::Up => energized_direction.contains(&Direction::Up),
                Direction::Down => energized_direction.contains(&Direction::Down),
            },
            MirrorType::Horizontal => match beam.direction {
                Direction::Up | Direction::Down => {
                    energized_direction
                        .iter()
                        .any(|d| d == &Direction::Up || d == &Direction::Down)
                        || (energized_direction.contains(&Direction::Left)
                            && energized_direction.contains(&Direction::Right))
                }
                Direction::Left => energized_direction.contains(&Direction::Left),
                Direction::Right => energized_direction.contains(&Direction::Right),
            },
        };
    } else {
        return false;
    }
}
fn next_move(beam: &mut Beam, mirror: &MirrorType, energized: &mut HashMap<IVec2, Vec<Direction>>) {
    match mirror {
        MirrorType::Forwardslash => match beam.direction {
            Direction::Right => {
                beam.location.y -= 1;
                beam.direction = Direction::Up;
            }
            Direction::Left => {
                beam.location.y += 1;
                beam.direction = Direction::Down;
            }
            Direction::Up => {
                beam.location.x += 1;
                beam.direction = Direction::Right;
            }
            Direction::Down => {
                beam.location.x -= 1;
                beam.direction = Direction::Left;
            }
        },
        MirrorType::Backslash => match beam.direction {
            Direction::Right => {
                beam.location.y += 1;
                beam.direction = Direction::Down
            }
            Direction::Left => {
                beam.location.y -= 1;
                beam.direction = Direction::Up
            }
            Direction::Up => {
                beam.location.x -= 1;
                beam.direction = Direction::Left
            }
            Direction::Down => {
                beam.location.x += 1;
                beam.direction = Direction::Right
            }
        },
        MirrorType::Vertical => match beam.direction {
            Direction::Right => {
                beam.location.y += 1;
                beam.direction = Direction::Down;

                //Add new beam goes up
            }
            Direction::Left => {
                beam.location.y -= 1;
                beam.direction = Direction::Up;
            } //TODO
            Direction::Up => beam.location.y -= 1,
            Direction::Down => beam.location.y += 1,
        },
        MirrorType::Horizontal => match beam.direction {
            Direction::Right => {
                beam.location.x += 1;
            }
            Direction::Left => {
                beam.location.x -= 1;
            }
            Direction::Up => {
                beam.location.x -= 1;
                beam.direction = Direction::Left;
            } //TODO
            Direction::Down => {
                beam.location.x += 1;
                beam.direction = Direction::Right;
            } //TODO
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!("46", process(input))
    }
}
