fn main() {
    let input = include_str!("../input.txt");
    // read input line by line, two characters separated by a space
    let mut score = 0;
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let opponent = chars.next().unwrap();
        let mut player = chars.nth(1).unwrap();
        // rock = X, paper = Y, scissors = Z
        // rock = A, paper = B, scissors = C
        // uncomment this block for part 1
        player = match opponent {
            'A' => match player {
                'X' => 'Z',
                'Y' => 'X',
                'Z' => 'Y',
                _ => player,
            },
            'B' => match player {
                'X' => 'X',
                'Y' => 'Y',
                'Z' => 'Z',
                _ => player,
            },
            'C' => match player {
                'X' => 'Y',
                'Y' => 'Z',
                'Z' => 'X',
                _ => player,
            },
            _ => player,
        };
        score += match (opponent, player) {
            ('A', 'Y') => 6,
            ('A', 'Z') => 0,
            ('B', 'X') => 0,
            ('B', 'Z') => 6,
            ('C', 'X') => 6,
            ('C', 'Y') => 0,
            _ => 3,
        };

        score += match player {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };
    });
    println!("{}", score);
}
