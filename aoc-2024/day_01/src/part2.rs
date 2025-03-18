use std::collections::HashMap;




fn parse_input(input: &str) -> String {
    let (vec1, vec2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let nums: Vec<u32> = line.split_whitespace()
                                      .filter_map(|s| s.parse::<u32>().ok())
                                      .collect();
            if nums.len() == 2 {
                Some((nums[0], nums[1]))
            } else {
                None
            }
        })
        .unzip(); // Unzip the tuples into two separate vectors
    // println!("{:?} {:?}", vec1, vec2);
    let mut occurrences = HashMap::new();
    vec1.iter().map(|&value| {
        if occurrences.contains_key(&value) == false {
            occurrences.entry(value).insert_entry(vec2.iter().filter(|&&v| v == value).count() as u32);
        }
        value * occurrences.get(&value).unwrap()
    }).sum::<u32>().to_string()
}


pub fn process(input: &str) -> String {
    parse_input(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input))
    }
}
