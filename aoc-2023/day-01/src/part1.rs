pub fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let number: u32 = lines
        .iter()
        .map(|&line| -> u32 {
            let chars: Vec<char> = line.chars().collect();
            //Find start from begin
            let first_number = chars.iter().find(|&char| char.is_numeric()).unwrap();
            //Find start from end
            let last_number = chars.iter().rev().find(|&char| char.is_numeric()).unwrap();
            first_number.to_digit(10).unwrap() * 10 + last_number.to_digit(10).unwrap()
        })
        .sum();
    number.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142", process(input))
    }
}
