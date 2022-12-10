use std::collections::HashMap;

fn main() {
    let lines = include_str!("../input.txt").lines();
    let mut folders: HashMap<String, u32> = HashMap::new();
    let mut current_path = vec![];
    let mut root_size = 0;
    lines.for_each(|line| {
        if line.starts_with("$") && line.contains("cd") {
            let arg = line.split(" ").nth(2).unwrap();
            if "/" == arg {
                current_path.push("root");
            } else if ".." == arg {
                current_path.pop();
            } else {
                current_path.push(arg);
                folders.insert(current_path.join("/"), 0);
            }
        } else if !line.starts_with("$") {
            if line.starts_with("dir") {
                return;
            }
            let size = line.split(" ").nth(0).unwrap().parse::<u32>().unwrap();
            root_size += size;
            let mut path_list = vec![];
            current_path.iter().for_each(|p| {
                path_list.push(p.to_string());
                let item = path_list.join("/");
                folders.entry(item).and_modify(|e| *e += size);
            });
        }
    });
    let mut part1 = 0;
    let mut part2: Vec<u32> = vec![];
    let unused_space = 70000000 - root_size;
    folders.iter().for_each(|(_, v)| {
        if v < &100000 {
            part1 += v;
        }
        if v + unused_space > 30000000 {
            part2.push(*v);
        }
    });

    println!("part1: {}", part1);
    println!("part2: {}", part2.iter().min().unwrap());
}
