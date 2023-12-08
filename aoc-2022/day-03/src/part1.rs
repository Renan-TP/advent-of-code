use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack<'a> {
    first_compartment: &'a str,
    second_compartment: &'a str,
}

impl<'a> Rucksack<'a> {
    fn compare(&self) -> char {
        let mut result: char = '\0';
        self.first_compartment.chars().any(|fc| {
            self.second_compartment.chars().any(|sc| {
                if sc == fc {
                    result = sc;
                    return true;
                }
                false
            })
        });
        result
    }
    fn look_up(&self, c: char, lookup_set: &HashSet<(char, u32)>) -> u32 {
        let (_, result) = lookup_set.iter().find(|(ch, _)| c == *ch).expect("found");
        *result
    }
}

fn parse_normal(input: &str) -> Vec<Rucksack> {
    let lines = input.lines().collect::<Vec<&str>>();
    lines
        .iter()
        .map(|&line| {
            let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
            Rucksack {
                first_compartment,
                second_compartment,
            }
        })
        .collect::<Vec<Rucksack>>()
}

pub fn process(input: &str) -> String {
    let rucksacks = parse_normal(input);
    // Map the letters to numbers
    let letters = 'a'..='z';
    let upper_letters = 'A'..='Z';
    let numbers = 1..=26;
    let upper_numbers = 27..=52;
    let mut hash = letters.zip(numbers).collect::<HashSet<(char, u32)>>();
    hash.extend(
        upper_letters
            .zip(upper_numbers)
            .collect::<HashSet<(char, u32)>>(),
    );
    rucksacks
        .iter()
        .map(|r| r.look_up(r.compare(), &hash))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!("157", process(input))
    }
}
