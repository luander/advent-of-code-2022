use std::cmp;
use std::collections::HashSet;

#[derive(Debug)]
struct Rope2 {
    rope: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Rope2 {
    fn from_str(input: &str) -> Rope2 {
        let mut rope = Rope2 {
            visited: HashSet::new(),
            rope: vec![(0, 0); 10],
        };
        rope.visited.insert((0, 0));
        input.lines().for_each(|line| {
            let mut iter = line.split_whitespace();
            let direction = iter.next().unwrap().chars().next().unwrap();
            let distance = iter.next().unwrap().parse::<usize>().unwrap();
            rope.mov(direction, distance);
        });
        rope
    }

    fn part2(&self) -> usize {
        self.visited.len()
    }

    fn tail_move(&mut self, from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
        let delta_x: i32 = to.0 - from.0;
        let delta_y: i32 = to.1 - from.1;
        let mut move_by_x = 0;
        let mut move_by_y = 0;
        if (delta_x.abs() <= 2) && (delta_y.abs() <= 2) {
            move_by_x = delta_x.clamp(-1, 1);
            move_by_y = delta_y.clamp(-1, 1);
        } else if delta_x.abs() == 2 && delta_y == 0 {
            move_by_x = delta_x.clamp(-1, 1);
        } else if delta_x == 0 && delta_y.abs() == 2 {
            move_by_y = delta_y.clamp(-1, 1);
        }
        (move_by_x, move_by_y)
    }

    fn distance(&self, a: (i32, i32), b: (i32, i32)) -> i32 {
        // manhattan distance
        cmp::max((a.0 - b.0).abs(), (a.1 - b.1).abs())
    }

    fn follow_head(&mut self) {
        for i in 1..self.rope.len() {
            if self.distance(self.rope[i - 1], self.rope[i]) > 1 {
                let delta = self.tail_move(self.rope[i], self.rope[i - 1]);
                self.rope[i].0 += delta.0;
                self.rope[i].1 += delta.1;
                if i == 9 {
                    self.visited.insert(self.rope[i]);
                }
            }
        }
    }

    fn mov(&mut self, direction: char, steps: usize) {
        match direction {
            'U' => {
                for _ in 0..steps {
                    self.rope[0].1 += 1; // move head
                    self.follow_head();
                }
            }
            'D' => {
                for _ in 0..steps {
                    self.rope[0].1 -= 1; // move head
                    self.follow_head();
                }
            }
            'L' => {
                for _ in 0..steps {
                    self.rope[0].0 -= 1; // move head
                    self.follow_head();
                }
            }
            'R' => {
                for _ in 0..steps {
                    self.rope[0].0 += 1; // move head
                    self.follow_head();
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn from_str(input: &str) -> Rope {
        let mut rope = Rope {
            visited: HashSet::new(),
            head: (0, 0),
            tail: (0, 0),
        };
        rope.visited.insert((0, 0));
        input.lines().for_each(|line| {
            let mut iter = line.split_whitespace();
            let direction = iter.next().unwrap().chars().next().unwrap();
            let distance = iter.next().unwrap().parse::<usize>().unwrap();
            rope.mov(direction, distance);
        });
        rope
    }

    fn part1(&self) -> usize {
        self.visited.len()
    }

    fn tail_move(&mut self) {
        let delta_x: i32 = self.head.0 - self.tail.0;
        let delta_y: i32 = self.head.1 - self.tail.1;
        if (delta_x.abs() <= 2) && (delta_y.abs() <= 2) {
            self.tail.0 += delta_x.clamp(-1, 1);
            self.tail.1 += delta_y.clamp(-1, 1);
        } else if delta_x.abs() == 2 && delta_y == 0 {
            self.tail.0 += delta_x.clamp(-1, 1);
        } else if delta_x == 0 && delta_y.abs() == 2 {
            self.tail.1 += delta_y.clamp(-1, 1);
        }
    }

    fn distance(&self) -> i32 {
        // manhattan distance
        cmp::max(
            (self.head.0 - self.tail.0).abs(),
            (self.head.1 - self.tail.1).abs(),
        )
    }

    fn follow_head(&mut self) {
        if self.distance() > 1 {
            self.tail_move();
            self.visited.insert(self.tail);
        }
    }

    fn mov(&mut self, direction: char, steps: usize) {
        match direction {
            'U' => {
                for _ in 0..steps {
                    self.head.1 += 1; // move head
                    self.follow_head();
                }
            }
            'D' => {
                for _ in 0..steps {
                    self.head.1 -= 1; // move head
                    self.follow_head();
                }
            }
            'L' => {
                for _ in 0..steps {
                    self.head.0 -= 1; // move head
                    self.follow_head();
                }
            }
            'R' => {
                for _ in 0..steps {
                    self.head.0 += 1; // move head
                    self.follow_head();
                }
            }
            _ => {}
        }
    }
}
fn main() {
    let input = include_str!("../input.txt");
    let result = Rope::from_str(input);
    println!("{}", result.part1());
    let result = Rope2::from_str(input);
    println!("{}", result.part2());
}

#[cfg(test)]
mod tests {
    // import the code we want to test
    use super::*;
    const INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn sample1_should_be_13() {
        let rope = Rope::from_str(INPUT1);
        assert_eq!(13, rope.part1());
    }

    #[test]
    fn sample2_should_be_36() {
        let rope = Rope2::from_str(INPUT2);
        assert_eq!(36, rope.part2());
    }
}
