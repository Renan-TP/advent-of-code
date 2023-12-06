use day_02::part12_nom::{process, process_2};

fn main() {
    let file = include_str!("../../input1.txt");
    let result1 = process(file);
    let result2 = process_2(file);
    println!("{result1} | {result2}");
}
