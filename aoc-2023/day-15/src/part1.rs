fn hash_data(input: &str, current_value: u8) -> u8 {
    input.chars().fold(current_value as u32, |mut value, c| {
        let c_value = c as u8;
        value += c_value as u32;
        value *= 17;
        value %= 256;
        value
    }) as u8
}

fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

pub fn process(input: &str) -> String {
    let data = parse(input);
    // dbg!(&data);
    // data.into_iter()
    //     .fold(0u8, |mut current_value, s| {
    //         current_value = hash_data(s, current_value);
    //         current_value
    //     })
    //     .to_string()
    data.into_iter()
        .map(|s| hash_data(s, 0) as u64)
        .sum::<u64>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn small_test(#[case] input: &str, #[case] expected: u8) {
        assert_eq!(expected, hash_data(input, 0))
    }

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input))
    }
}
