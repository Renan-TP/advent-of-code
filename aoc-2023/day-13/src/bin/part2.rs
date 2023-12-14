use day_13::part2::process;

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{result}");
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{result}");
}
