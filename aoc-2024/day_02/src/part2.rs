// use std::collections::HashSet;

#[derive(Debug)]
enum LevelSafety {
    Safe,
    Unsafe,
}

// Function fixing the unsafe levels by remove 1 element
fn fix_unsafe_level(level: &Vec<u32>) -> Vec<u32> {
    let unsafe_level = level.clone();
    let mut unsafe_level_index = 0;
    let mut unsafe_level_fixed = unsafe_level.clone();
    unsafe_level.iter().enumerate().for_each(|(i, &_x)| {
        let mut unsafe_level_temp = unsafe_level.clone();
        unsafe_level_temp.remove(i);
        if let (LevelSafety::Safe, _) = safety_check(&unsafe_level_temp) {
            unsafe_level_fixed = unsafe_level_temp.clone();
            unsafe_level_index = i;
        }
    });
    unsafe_level_fixed
}

fn safety_check(level: &Vec<u32>) -> (LevelSafety, usize) {
    let mut increase_sorted_level = level.clone();
    increase_sorted_level.sort();
    let mut decrease_sorted_level = level.clone();
    decrease_sorted_level.sort_by(|a, b| b.cmp(a));
    // Compare the sorted level with the original level
    // If they are the same, then the level is safe
    // If they are different, then the level is unsafe
    if increase_sorted_level == *level || decrease_sorted_level == *level {
        // Check if there are any duplicates in the level
        // If there are duplicates, then the level is unsafe
        // If there are no duplicates, then the level is safe
        let mut level_set = level.clone();
        level_set.sort();
        level_set.dedup();
        // If the length of the level is the same as the length of the level set
        if level_set.len() == level.len() {
            if level
                .iter()
                .take(level.len() - 1)
                .enumerate()
                .any(|(i, &x)| {
                    let diff = x.abs_diff(level[i + 1]);
                    !(1..=3).contains(&diff)
                })
            {
                (LevelSafety::Unsafe, level.len())
            } else {
                (LevelSafety::Safe, level.len())
            }
        } else {
            // If no duplicates are found, return Safe
            (LevelSafety::Unsafe, level.len())
        }
    } else {
        (LevelSafety::Unsafe, 0)
    }
}

pub fn process(input: &str) -> String {
    let lines = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|adj_level| adj_level.parse::<u32>().ok().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    // println!("{:?}", lines);
    let unsafe_level = lines
        .iter()
        .filter(|&level| match safety_check(level) {
            (LevelSafety::Safe, _) => {
                // println!("Safe {:?}", level);
                false
            }
            (LevelSafety::Unsafe, _) => {
                // println!("Unsafe {:?}", level);
                true
            }
        })
        .filter_map(|level| {
            let fixed_level = fix_unsafe_level(level);
            match safety_check(&fixed_level) {
                (LevelSafety::Safe, _) => None,
                (LevelSafety::Unsafe, _) => Some(fixed_level),
            }
        })
        .count();
    println!("{:?}", lines.len());
    println!("{:?}", unsafe_level);
    (lines.len() - unsafe_level).to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input))
    }
}
