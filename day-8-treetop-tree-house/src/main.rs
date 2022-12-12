struct TreeMap {
    trees: Vec<Vec<u32>>,
}

impl TreeMap {
    fn from_str(input: &str) -> Self {
        let data: Vec<Vec<u32>> = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        Self { trees: data }
    }

    fn part1(&self) -> usize {
        self.trees
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(y, _)| self.isvisible(x, *y))
                    .count()
            })
            .sum()
    }

    fn part2(&self) -> usize {
        let mut best_score = 0;
        self.trees
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, _)| self.scenic_score(x, y))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        if x == 0 || y == 0 || x == self.trees.len() - 1 || y == self.trees[x].len() - 1 {
            return 0;
        }
        let len = self.trees[x].len();
        let up = self.trees[1..x]
            .iter()
            .rev()
            .map(|item| item[y])
            .map_while(|item| {
                if item < self.trees[x][y] {
                    Some(item)
                } else {
                    None
                }
            })
            .count()
            + 1;
        let down = self.trees[x + 1..len - 1]
            .iter()
            .map(|item| item[y])
            .map_while(|item| {
                if item < self.trees[x][y] {
                    Some(item)
                } else {
                    None
                }
            })
            .count()
            + 1;
        let left = self.trees[x][1..y]
            .iter()
            .rev()
            .map_while(|item| {
                if item < &self.trees[x][y] {
                    Some(item)
                } else {
                    None
                }
            })
            .count()
            + 1;
        let right = self.trees[x][y + 1..len - 1]
            .iter()
            .map_while(|item| {
                if item < &self.trees[x][y] {
                    Some(item)
                } else {
                    None
                }
            })
            .count()
            + 1;
        println!("{} {} {} {}", up, left, right, down);
        up * left * right * down
    }

    fn isvisible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.trees.len() - 1 || y == self.trees[x].len() - 1 {
            return true;
        }
        let up = self.trees[..x].iter().map(|item| item[y]).max().unwrap() < self.trees[x][y];
        let down = self.trees[x + 1..]
            .iter()
            .map(|item| item[y])
            .max()
            .unwrap()
            < self.trees[x][y];
        let left = *self.trees[x][..y].iter().max().unwrap() < self.trees[x][y];
        let right = *self.trees[x][y + 1..].iter().max().unwrap() < self.trees[x][y];
        up || left || right || down
    }
}

fn main() {
    let treemap = TreeMap::from_str(include_str!("../input.txt"));
    println!("part1 {}", treemap.part1());
    println!("part2 {}", treemap.part2());
}
