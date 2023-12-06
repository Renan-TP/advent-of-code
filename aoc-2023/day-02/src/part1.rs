#[derive(Default, Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}
#[derive(Debug)]
struct Game {
    id: u32,
    // grabs: [Cubes; 3],
    valid: bool,
}
pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| -> Game {
            let t = line.split(':').collect::<Vec<&str>>();
            let &id = t
                .first()
                .unwrap()
                .split_whitespace()
                .filter_map(|id| id.parse::<u32>().ok())
                .collect::<Vec<u32>>()
                .first()
                .unwrap();
            let grabs = t.last().unwrap().trim().split(';').collect::<Vec<&str>>();
            let mut cubes: [Cubes; 3] = Default::default();
            let mut index: usize = 0;
            let is_valid = grabs
                .iter()
                .filter(|&s| -> bool {
                    let mut colors_and_number = s.split(',').collect::<Vec<&str>>();
                    // dbg!(&colors_and_number);
                    while let Some(color_and_number) = colors_and_number.pop() {
                        let color_and_number =
                            color_and_number.split_whitespace().collect::<Vec<&str>>();
                        // dbg!(&color_and_number);
                        // dbg!(color_and_number.first().unwrap().parse::<u32>().unwrap());
                        // dbg!(index);
                        if color_and_number.last().unwrap().contains("red") {
                            cubes[index.min(2)].red =
                                color_and_number.first().unwrap().parse::<u32>().unwrap();
                            if color_and_number.first().unwrap().parse::<u32>().unwrap() > 12 {
                                index += 1;
                                return false;
                            }
                        }
                        if color_and_number.last().unwrap().contains("green") {
                            cubes[index.min(2)].green =
                                color_and_number.first().unwrap().parse::<u32>().unwrap();
                            if color_and_number.first().unwrap().parse::<u32>().unwrap() > 13 {
                                index += 1;
                                return false;
                            }
                        }
                        if color_and_number.last().unwrap().contains("blue") {
                            cubes[index.min(2)].blue =
                                color_and_number.first().unwrap().parse::<u32>().unwrap();
                            if color_and_number.first().unwrap().parse::<u32>().unwrap() > 14 {
                                index += 1;
                                return false;
                            }
                        }
                    }
                    index += 1;
                    true
                })
                .count()
                .eq(&grabs.len());
            // dbg!(&cubes);
            Game {
                id,
                // grabs: cubes,
                valid: is_valid,
            }
        })
        .filter(|game| game.valid)
        .map(|game| game.id)
        .sum::<u32>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input))
    }
}
