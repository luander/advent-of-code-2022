fn main() {
    let input = include_str!("../input.txt");
    let mut sum: Vec<i32> = Vec::new();
    let mut cur_cal = 0;
    input.lines().for_each(|line| {
        let cal: i32 = line.parse().unwrap_or(-1);
        if cal != -1 {
            cur_cal += cal;
        } else {
            sum.push(cur_cal);
            cur_cal = 0;
        }
    });
    sum.sort_unstable_by(|a, b| b.cmp(a));
    println!("part1({}), part2({})", sum[0] + sum[1] + sum[2], sum[0]);
}
