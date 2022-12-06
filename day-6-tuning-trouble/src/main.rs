use itertools::Itertools;

fn calculate(input: &str, len: usize) -> usize {
    let mut j = 0;
    let mut result = 0;
    for i in len..input.len() {
        if !input[j..i].chars().all_unique() {
            j += 1;
        } else {
            result = i;
            break;
        }
    }
    result
}
fn main() {
    let input = include_str!("../input.txt");
    let answer_part1 = calculate(input, 4);
    println!("part1: {}", answer_part1);
    let answer_part2 = calculate(input, 14);
    println!("part2: {}", answer_part2);
}
