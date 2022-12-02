use std::io;

fn main() {
    let (score1, score2) =
        io::stdin()
            .lines()
            .map(|line| line.unwrap())
            .fold((0, 0), |(acc1, acc2), line| {
                let mut signs = line.split_whitespace();
                let sign1 = signs.next().unwrap().chars().next().unwrap();
                let sign2_1 = signs.next().unwrap().chars().next().unwrap();
                let sign2_2 = result_to_sign(sign1, sign2_1);
                return (
                    acc1 + score_win(sign1, sign2_1) + score_sign(sign2_1),
                    acc2 + score_win(sign1, sign2_2) + score_sign(sign2_2),
                );
            });

    println!("{}", score1);
    println!("{}", score2);
}

fn score_win(sign1: char, sign2: char) -> i32 {
    match (sign1, sign2) {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
        _ => panic!("Invalid input: {} {}", sign1, sign2),
    }
}

fn score_sign(sign: char) -> i32 {
    match sign {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid input: {}", sign),
    }
}

fn result_to_sign(sign1: char, sign2: char) -> char {
    match (sign1, sign2) {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',
        ('B', 'X') => 'X',
        ('B', 'Y') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',
        _ => panic!("Invalid input: {} {}", sign1, sign2),
    }
}
