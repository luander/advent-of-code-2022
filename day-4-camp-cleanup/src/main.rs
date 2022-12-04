fn calculate(input: &str) -> i32 {
    let case: Vec<&str> = input.split(&['-', ','][..]).collect();
    let i1: i32 = case[0].parse().unwrap();
    let i2: i32 = case[1].parse().unwrap();
    let j1: i32 = case[2].parse().unwrap();
    let j2: i32 = case[3].parse().unwrap();

    // Not intersecting
    if i2 < j1 || i1 > j2 {
        return 0;
    }

    // i is fully contained
    if i1 >= j1 && i2 <= j2 {
        return 1;
    }

    // j is fully contained
    if j1 >= i1 && j2 <= i2 {
        return 1;
    }

    0
}

fn main() {
    let mut fully_contain = 0;

    include_str!("../input.txt")
        .lines()
        .for_each(|line| fully_contain += calculate(line));

    println!("{}", fully_contain);
}
