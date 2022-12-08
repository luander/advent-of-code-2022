fn shared_chars(first: &str, second: &str) -> String {
    let shared_chars = first
        .chars()
        .filter(|a| second.chars().any(|b| a == &b))
        .collect::<String>();
    shared_chars.chars().collect()
}

fn shared_char(first: &str, second: &str) -> i32 {
    shared_chars(first, second).chars().nth(0).unwrap() as i32
}

fn map_range(value: i32, lower_base: i32, upper_base: i32) -> i32 {
    if 97 <= value && value <= 122 {
        (value - 97) + lower_base
    } else if 65 <= value && value <= 90 {
        (value - 65) + upper_base
    } else {
        0
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut part1: i32 = 0;
    input.lines().for_each(|line| {
        let (first, second) = line.split_at(line.len() / 2);
        let shared_char = shared_char(first, second);
        part1 += map_range(shared_char, 1, 27) as i32;
    });
    // divide the number of lines by 3 to get the number of jumps
    let jumps = input.lines().count() / 3;
    let mut lines_iter = input.lines();
    let mut part2 = 0;
    for _ in 0..jumps {
        let first = lines_iter.next().unwrap();
        let second = lines_iter.next().unwrap();
        let third = lines_iter.next().unwrap();
        let shared_char = shared_chars(shared_chars(first, second).as_str(), third);
        part2 += map_range(shared_char.chars().nth(0).unwrap() as i32, 1, 27) as i32;
    }

    println!("{}", part1);
    println!("{}", part2);
}
