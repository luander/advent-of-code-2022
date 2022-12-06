use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let (crates, cmds) = input.split_once("\n\n").unwrap();
    let n = (input.lines().next().unwrap().len() / 4) + 1;
    let mut stacks = vec![vec![]; n];
    crates.lines().for_each(|line| {
        line.char_indices()
            .filter(|(_, symbol)| symbol.is_alphabetic())
            .for_each(|(i, symbol)| stacks[i / 4].push(symbol))
    });
    stacks.iter_mut().for_each(|stack| stack.reverse());

    let mut moved_crates = vec![];
    cmds.lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .map(|(amount, from, to)| (amount, from - 1, to - 1))
                .unwrap()
        })
        .for_each(|(amount, from, to)| {
            let new_len = stacks[from].len() - amount;
            moved_crates.extend(stacks[from].drain(new_len..));
            // moved_crates.reverse();
            stacks[to].append(&mut moved_crates);
        });
    let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    print!("{}", result);
}
