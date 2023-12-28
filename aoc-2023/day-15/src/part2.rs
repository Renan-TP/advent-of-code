
enum Operation {
    Remove,
    AddOrReplace,
}

impl Operation {

}
#[derive(Debug)]
struct Step<'a> {
    label: &'a str,
    focal_length: Option<usize>,
}

fn hash_data(input: &str, current_value: u8) -> u8 {
    input.chars().fold(current_value as u32, |mut value, c| {
        let c_value = c as u8;
        value += c_value as u32;
        value *= 17;
        value %= 256;
        value
    }) as u8
}

fn push_in_box_of_vec<'a>(step: Step<'a>, boxs: &mut Vec<Vec<(&'a str, usize)>>) {
    let box_number = hash_data(step.label, 0) as usize;
    match step.focal_length {
        Some(focal_lenght) => match is_box_have_label(step.label, &boxs[box_number]) {
            true => replace_focal_lenght(step.label, focal_lenght, &mut boxs[box_number]),
            false => boxs[box_number].push((step.label, focal_lenght)),
        },
        None => {
            match boxs[box_number]
                .iter()
                .enumerate()
                .find(|(_, (label, _))| *label == step.label)
            {
                Some((index, _)) => {
                    boxs[box_number].remove(index);
                }
                None => println!("Nothing to remove"),
            }
        }
    }
}
fn is_box_have_label<'a>(label: &'a str, the_box: &Vec<(&'a str, usize)>) -> bool {
    the_box.into_iter().any(|(l, _)| *l == label)
}
fn replace_focal_lenght(label: &str, focal_lenght: usize, the_box: &mut Vec<(&str, usize)>) {
    *the_box = the_box
        .iter()
        .map(|(k, v)| {
            if *k == label {
                (*k, focal_lenght)
            } else {
                (*k, *v)
            }
        })
        .collect::<Vec<(&str, usize)>>();
}
fn parse(input: &str) -> Vec<Step> {
    input
        .split(',')
        .map(|s| {
            if s.contains('=') {
                let mut s = s.split("=");
                Step {
                    label: s.next().expect("label"),
                    focal_length: Some(
                        s.next()
                            .expect("focal lenght")
                            .parse::<usize>()
                            .expect("parse focal lenght should be ok"),
                    ),
                }
            } else {
                let s = s.trim_end_matches('-');
                Step {
                    label: s,
                    focal_length: None,
                }
            }
        })
        .collect()
}

fn calculate_vecmap(boxs: &mut Vec<Vec<(&str, usize)>>) -> usize {
    boxs.into_iter()
        .enumerate()
        .filter(|(_, map)| !map.is_empty())
        .map(|(box_number, map)| {
            map.into_iter()
                .enumerate()
                .map(|(slot, (_, v))| {
                    dbg!((&box_number + 1, &slot + 1, *v as usize));
                    (box_number + 1) * (slot + 1) * (*v as usize)
                })
                .sum::<usize>()
        })
        .sum()
}
pub fn process(input: &str) -> String {
    let data = parse(input);
    // dbg!(&data);

    // let mut boxs: Vec<BTreeMap<&str, usize>> = vec![BTreeMap::new(); 256];
    let mut boxs: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    data.into_iter()
        .for_each(|i| push_in_box_of_vec(i, &mut boxs));
    dbg!(&boxs[0], &boxs[3]);
    // calculate_hashmap(&mut boxs).to_string()
    calculate_vecmap(&mut boxs).to_string()
    // todo!()
}
#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;

    // #[rstest]
    // #[case("HASH", 52)]
    // #[case("rn=1", 30)]
    // #[case("cm-", 253)]
    // #[case("qp=3", 97)]
    // #[case("cm=2", 47)]
    // #[case("qp-", 14)]
    // #[case("pc=4", 180)]
    // #[case("ot=9", 9)]
    // #[case("ab=5", 197)]
    // #[case("pc-", 48)]
    // #[case("pc=6", 214)]
    // #[case("ot=7", 231)]
    // fn small_test(#[case] input: &str, #[case] expected: u8) {
    //     assert_eq!(expected, hash_data(input, 0))
    // }

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input))
    }
}
