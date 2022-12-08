use std::cmp;
use std::io::{self, BufRead};

fn main() {
    let input = read_input();
    let rows = input.len();
    let columns = input[0].len();

    let mut cnt_1 = 0;
    let mut max_2 = 0;
    for i in 0..rows {
        for j in 0..columns {
            if check(&input, i, j) {
                cnt_1 += 1;
            }
            max_2 = cmp::max(max_2, score(&input, i, j));
        }
    }

    println!("{}", cnt_1);
    println!("{}", max_2);
}

fn check(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let rows = input.len();
    let columns = input[0].len();
    let height = input[x][y];

    if !(0..x).any(|i| input[i][y] >= height) {
        return true;
    }
    if !((x + 1)..rows).any(|i| input[i][y] >= height) {
        return true;
    }
    if !(0..y).any(|i| input[x][i] >= height) {
        return true;
    }
    if !((y + 1)..columns).any(|i| input[x][i] >= height) {
        return true;
    }

    return false;
}

fn score(input: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let rows = input.len();
    let columns = input[0].len();
    let height = input[x][y];

    let mut up = 0;
    for i in (0..x).rev() {
        if input[i][y] < height {
            up += 1;
        }
        if input[i][y] == height {
            up += 1;
            break;
        }
        if input[i][y] > height {
            break;
        }
    }

    let mut down = 0;
    for i in (x + 1)..rows {
        if input[i][y] < height {
            down += 1;
        }
        if input[i][y] == height {
            down += 1;
            break;
        }
        if input[i][y] > height {
            break;
        }
    }

    let mut left = 0;
    for i in (0..y).rev() {
        if input[x][i] < height {
            left += 1;
        }
        if input[x][i] == height {
            left += 1;
            break;
        }
        if input[x][i] > height {
            break;
        }
    }

    let mut right = 0;
    for i in (y + 1)..columns {
        if input[x][i] < height {
            right += 1;
        }
        if input[x][i] == height {
            right += 1;
            break;
        }
        if input[x][i] > height {
            break;
        }
    }

    return up * down * left * right;
}

fn read_input() -> Vec<Vec<char>> {
    return io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();
}
