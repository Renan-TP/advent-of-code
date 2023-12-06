fn word_replace(line: &str) -> String {
    let words = [
        "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let replacement_words = [
        "0", "o1ne", "t2wo", "t3hree", "f4our", "f5ive", "s6ix", "s7even", "e8ight", "n9ine",
    ];
    line.replace(words[0], replacement_words[0])
        .replace(words[1], replacement_words[1])
        .replace(words[2], replacement_words[2])
        .replace(words[3], replacement_words[3])
        .replace(words[4], replacement_words[4])
        .replace(words[5], replacement_words[5])
        .replace(words[6], replacement_words[6])
        .replace(words[7], replacement_words[7])
        .replace(words[8], replacement_words[8])
        .replace(words[9], replacement_words[9])
}
// fn word_to_number_from_start(line: &str) -> String {
//     let words = vec![
//         "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
//     ];
//     let find_result = words.iter().map(|&word| line.find(word)).enumerate();
//     let found_number: Vec<(usize, usize)> = find_result
//         .into_iter()
//         .filter(|(_, option)| option.is_some())
//         .map(|(found, option)| (found, option.unwrap()))
//         .collect();

//     if found_number.is_empty() {
//         return line.trim().to_string();
//     } else {
//         let word_index = found_number.into_iter().min_by_key(|&(_, a)| a).unwrap().0;
//         return word_to_number_from_start(
//             &line
//                 .replace(words[word_index], &word_index.to_string())
//                 .trim(),
//         );
//     }
//     // if let Some((index, value)) =  {
//     //     return line.replace(words[index], &index.to_string());
//     // line.replace("one", "1")
//     // .replace("two", "2")
//     // .replace("three", "3")
//     // .replace("four", "4")
//     // .replace("five", "5")
//     // .replace("six", "6")
//     // .replace("seven", "7")
//     // .replace("eight", "8")
//     // .replace("nine", "9")
//     // }
// }
// fn word_to_number_from_end(line: &str) -> String {
//     let words = vec![
//         "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
//     ];
//     let find_result = words.iter().map(|&word| line.find(word)).enumerate();
//     let found_number: Vec<(usize, usize)> = find_result
//         .into_iter()
//         .filter(|(_, option)| option.is_some())
//         .map(|(found, option)| (found, option.unwrap()))
//         .collect();

//     if found_number.is_empty() {
//         return line.trim().to_string();
//     } else {
//         let word_index = found_number.into_iter().max_by_key(|&(_, a)| a).unwrap().0;
//         return word_to_number_from_end(
//             &line
//                 .replace(words[word_index], &word_index.to_string())
//                 .trim(),
//         );
//     }
//     // if let Some((index, value)) =  {
//     //     return line.replace(words[index], &index.to_string());
//     // line.replace("one", "1")
//     // .replace("two", "2")
//     // .replace("three", "3")
//     // .replace("four", "4")
//     // .replace("five", "5")
//     // .replace("six", "6")
//     // .replace("seven", "7")
//     // .replace("eight", "8")
//     // .replace("nine", "9")
//     // }
// }
pub fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let number: u32 = lines
        .iter()
        .map(|&line| -> u32 {
            let chars_from_start: Vec<char> = word_replace(line).chars().collect();
            // dbg!(&chars);
            //Find start from begin
            let first_number = chars_from_start
                .iter()
                .find(|&char| char.is_numeric())
                .unwrap();
            //Find start from end
            let chars_from_end: Vec<char> = word_replace(line).chars().collect();
            let last_number = chars_from_end
                .iter()
                .rev()
                .find(|&char| char.is_numeric())
                .unwrap();

            first_number.to_digit(10).unwrap() * 10 + last_number.to_digit(10).unwrap()
            // dbg!(word_to_number(line), line_number);
        })
        .sum();
    number.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!("281", process(input))
    }
}
