struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        let mut parts = input.split_whitespace();
        let operation = Operation::from_str(parts.next().unwrap());
        let argument = parts.next().unwrap_or("0").parse::<i32>().unwrap();
        Instruction {
            operation,
            argument,
        }
    }
}

struct Operation {
    function: fn(&mut i32, i32),
    cycles: i32,
}

impl Operation {
    fn from_str(input: &str) -> Operation {
        match input {
            "addx" => Operation {
                function: |x, y| *x += y,
                cycles: 2,
            },
            "noop" => Operation {
                function: |_, _| {},
                cycles: 1,
            },
            _ => panic!("Unknown operation: {}", input),
        }
    }

    fn execute(&self, x: &mut i32, y: i32) {
        (self.function)(x, y);
    }
}

struct CathodeRayTube {
    instructions: Vec<Instruction>,
}

impl CathodeRayTube {
    fn from_str(input: &str) -> CathodeRayTube {
        CathodeRayTube {
            instructions: input
                .lines()
                .map(|line| Instruction::from_str(line))
                .collect(),
        }
    }

    fn draw(&self, cycle: i32, instruction: &Instruction, acc: i32) {
        for i in 0..instruction.operation.cycles {
            // println!("{} {}", (cycle + i).rem_euclid(40), acc);
            let pos = (cycle + i).rem_euclid(40);
            if pos == acc || pos == acc - 1 || pos == acc + 1 {
                print!("#");
            } else {
                print!(".");
            }
            if pos == 39 {
                println!();
            }
        }
    }

    fn execute_program(&self) -> i32 {
        let mut cycles: i32 = 0;
        let mut result = 0;
        let mut multiplier = 1;
        let gold_cycles = vec![20, 60, 100, 140, 180, 220];
        self.instructions.iter().fold(1, |mut acc, instruction| {
            self.draw(cycles, instruction, acc);
            cycles += instruction.operation.cycles;
            if cycles / 20 == multiplier {
                let golden_cycle = 20 * multiplier;
                let strength = golden_cycle * acc;
                if gold_cycles.contains(&golden_cycle) {
                    result += strength;
                    // println!("{} {}", golden_cycle, strength);
                }
                multiplier += 1;
            }
            // println!(
            //     "{} {} {} {}",
            //     cycles, instruction.operation.name, instruction.argument, acc
            // );

            instruction
                .operation
                .execute(&mut acc, instruction.argument);
            acc
        });
        result
    }

    fn part1and2(&self) -> i32 {
        self.execute_program()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let cathode_ray_tube = CathodeRayTube::from_str(input);
    println!("Part 1: {}", cathode_ray_tube.part1and2());
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn sample1_should_be_13140() {
        let cathode_ray_tube = CathodeRayTube::from_str(INPUT1);
        assert_eq!(13140, cathode_ray_tube.part1and2());
    }
}
