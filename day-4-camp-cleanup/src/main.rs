use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate(input: String) -> i32 {
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
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut fully_contain = 0;

    for (_, line) in reader.lines().enumerate() {
        let case = line.unwrap();
        fully_contain += calculate(case);
    }
    println!("{}", fully_contain);
}
