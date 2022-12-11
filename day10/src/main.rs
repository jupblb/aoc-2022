use std::io;

use regex::Regex;

enum Op {
    NOOP,
    ADDX(i32),
}

fn main() {
    let mut cycle = 1;
    let mut x = 1;
    let mut res: [Option<i32>; 1000] = [None; 1000];
    res[0] = Some(1);

    read_input().iter().for_each(|op| {
        res[cycle] = Some(x);
        match op {
            Op::NOOP => cycle += 1,
            Op::ADDX(n) => {
                x += n;
                cycle += 2;
            }
        }
    });

    let mut signal_strength = 0;
    for i in 0..=5 {
        let index = 20 + i * 40;
        let value = check_last(res, index);
        signal_strength += (index as i32) * value;
    }

    println!("{}", signal_strength);

    let mut display = [[false; 40]; 6];
    for i in 0..6 {
        for j in 0..40 {
            let index = (i * 40 + j) as i32;
            let value = check_last(res, (index + 1) as usize);
            println!("{} - {}", index, value);
            let j = j as i32;
            if value == j - 1 || value == j || value == j + 1 {
                display[i][j as usize] = true;
            }
        }
    }

    for i in 0..6 {
        for j in 0..40 {
            if display[i][j] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn check_last(arr: [Option<i32>; 1000], idx: usize) -> i32 {
    for j in (0..=idx).rev() {
        if arr[j].is_some() {
            return arr[j].unwrap();
        }
    }
    return 0;
}

fn read_input() -> Vec<Op> {
    let addx = Regex::new(r"addx (-?\d+)").unwrap();
    return io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            if line == "noop" {
                return Some(Op::NOOP);
            }

            let cap = addx.captures(&line);
            if cap.is_some() {
                let num = cap
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                return Some(Op::ADDX(num));
            }
            return None;
        })
        .flatten()
        .collect();
}
