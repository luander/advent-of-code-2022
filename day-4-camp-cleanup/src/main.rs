fn calculate(input: &str) -> (i32, i32) {
    let case: Vec<&str> = input.split(&['-', ','][..]).collect();
    let i1: i32 = case[0].parse().unwrap();
    let i2: i32 = case[1].parse().unwrap();
    let j1: i32 = case[2].parse().unwrap();
    let j2: i32 = case[3].parse().unwrap();

    // Not intersecting
    if i2 < j1 || i1 > j2 {
        return (0, 1);
    }

    // i is fully contained
    if i1 >= j1 && i2 <= j2 {
        return (1, 0);
    }

    // j is fully contained
    if j1 >= i1 && j2 <= i2 {
        return (1, 0);
    }

    (0, 0)
}

fn main() {
    let mut part1 = 0;
    let mut total_lines = 0;
    let mut part2 = 0;

    include_str!("../input.txt").lines().for_each(|line| {
        let p1: i32;
        let p2: i32;
        (p1, p2) = calculate(line);
        part1 += p1;
        part2 += p2;
        total_lines += 1;
    });

    println!("{}, {}", part1, total_lines - part2);
}
