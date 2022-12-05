use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let _example: Vec<Vec<char>> = vec!["ZN", "MCD", "P"]
        .iter()
        .map(|stack| stack.chars().collect())
        .collect();
    let _input: Vec<Vec<char>> = vec![
        "STHFWR", "SGDQW", "BTW", "DRWTNQZJ", "FBHGLVTZ", "LPTCVBSG", "ZBRTWGP", "NGMTCJR", "LGBW",
    ]
    .iter()
    .map(|stack| stack.chars().collect())
    .collect();

    let input = read_input();
    solve1(&input, &mut _input.clone());
    solve2(&input, &mut _input.clone());
}

fn solve1(input: &Vec<(i32, i32, i32)>, vector: &mut Vec<Vec<char>>) {
    input.iter().for_each(|line| {
        for _ in 0..line.0 {
            let el = vector[(line.1 - 1) as usize].pop().unwrap();
            vector[(line.2 - 1) as usize].push(el);
        }
    });

    vector.iter().for_each(|stack| {
        stack.last().iter().for_each(|el| print!("{}", el));
    });
    println!();
}

fn solve2(input: &Vec<(i32, i32, i32)>, vector: &mut Vec<Vec<char>>) {
    input.iter().for_each(|line| {
        let els: Vec<char> = (0..line.0)
            .flat_map(|_| {
                return vector[(line.1 - 1) as usize].pop();
            })
            .collect();

        els.iter().rev().for_each(|el| {
            vector[(line.2 - 1) as usize].push(*el);
        });
    });

    vector.iter().for_each(|stack| {
        stack.last().iter().for_each(|el| print!("{}", el));
    });
    println!();
}

fn read_input() -> Vec<(i32, i32, i32)> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    return io::stdin()
        .lock()
        .lines()
        .flat_map(|line| {
            return re.captures(line.as_ref().unwrap()).map(|cap| {
                return (
                    cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                );
            });
        })
        .fold(Vec::<(i32, i32, i32)>::new(), |mut lines, line| {
            lines.push(line);
            return lines;
        });
}
