struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| -> Cubes {
            // let &id = t
            //     .first()
            //     .unwrap()
            //     .split_whitespace()
            //     .filter_map(|id| id.parse::<u32>().ok())
            //     .collect::<Vec<u32>>()
            //     .first()
            //     .unwrap();
            let mut minimum = Cubes {
                red: 0,
                green: 0,
                blue: 0,
            }; //Red, Green, Blue
            let grabs = line
                .split(':')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .trim()
                .split(';')
                .collect::<Vec<&str>>();
            grabs.iter().for_each(|&s| {
                let mut colors_and_number = s.split(',').collect::<Vec<&str>>();
                // dbg!(&colors_and_number);
                while let Some(color_and_number) = colors_and_number.pop() {
                    let color_and_number =
                        color_and_number.split_whitespace().collect::<Vec<&str>>();
                    // dbg!(&color_and_number);
                    // dbg!(color_and_number.first().unwrap().parse::<u32>().unwrap());
                    if color_and_number.last().unwrap().contains("red") {
                        minimum.red = minimum
                            .red
                            .max(color_and_number.first().unwrap().parse::<u32>().unwrap());
                    }
                    if color_and_number.last().unwrap().contains("green") {
                        minimum.green = minimum
                            .green
                            .max(color_and_number.first().unwrap().parse::<u32>().unwrap());
                    }
                    if color_and_number.last().unwrap().contains("blue") {
                        minimum.blue = minimum
                            .blue
                            .max(color_and_number.first().unwrap().parse::<u32>().unwrap());
                    }
                }
            });
            minimum
        })
        .map(|cubes| cubes.red * cubes.green * cubes.blue)
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
        assert_eq!("2286", process(input))
    }
}
