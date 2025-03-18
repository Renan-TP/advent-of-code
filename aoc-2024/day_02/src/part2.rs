#[derive(Debug)]
enum LevelSafety {
    Safe,
    Unsafe
}

fn safety_check(mut level: Vec<u32>) -> LevelSafety {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut first_remove = 0;
    match level.iter()
            .take(level.len()-1).enumerate()
            .any(|(n, &value)| {
                let diff = value.abs_diff(level[n+1]);
                match value.cmp(&level[n+1]) {
                    std::cmp::Ordering::Less => {
                        if !is_increasing{
                            first_remove = n + 1;
                            return true;
                        }
                        is_decreasing = false
                    },
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => {
                        if !is_decreasing {
                            first_remove = n;
                            return true;
                        }
                        is_increasing = false
                    },
                }
                if !(1..=3).contains(&diff) {
                    first_remove = n + 1;
                }
                !(1..=3).contains(&diff)
        }) {
        true => {
            is_increasing = true;
            is_decreasing = true;
            level.remove(first_remove);
            if level.iter()
            .take(level.len()-1).enumerate()
            .any(|(n, &value)| {
                let diff = value.abs_diff(level[n+1]);
                match value.cmp(&level[n+1]) {
                    std::cmp::Ordering::Less => is_decreasing = false,
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => is_increasing = false,
                }
                !(1..=3).contains(&diff)
        }) {
            return LevelSafety::Unsafe;
        }else {
            return LevelSafety::Safe;
        }
        }
        false => if is_increasing || is_decreasing 
        {
            return LevelSafety::Safe;
        },
    }
    LevelSafety::Unsafe
}


pub fn process(input: &str) -> String {
    let lines = input.lines()
        .map(
            |l| 
            l.split_whitespace().map(
                |adj_level| 
                adj_level.parse::<u32>().ok().unwrap()
            ).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    // println!("{:?}", lines);
    lines.iter().filter(|level|{
       
        match safety_check(level.to_vec()) {
        LevelSafety::Safe => {
            println!("Safe");
            true
        },
        LevelSafety::Unsafe => {
            println!("Unsafe");
            false
        }}
    }
    ).count().to_string()
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
