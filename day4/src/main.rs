use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let input = read_input();

    let overlaps_1: i32 = input
        .iter()
        .map(|((d1_1, d1_2), (d2_1, d2_2))| {
            if d1_1 <= d2_1 && d1_2 >= d2_2 {
                return 1;
            }
            if d2_1 <= d1_1 && d2_2 >= d1_2 {
                return 1;
            }
            return 0;
        })
        .sum();

    println!("{}", overlaps_1);

    let overlaps_2: i32 = input
        .iter()
        .map(|((d1_1, d1_2), (d2_1, d2_2))| {
            if d1_1 <= d2_1 && d2_1 <= d1_2 {
                return 1;
            }
            if d1_1 <= d2_2 && d2_2 <= d1_2 {
                return 1;
            }
            if d1_1 >= d2_1 && d1_2 <= d2_2 {
                return 1;
            }
            return 0;
        })
        .sum();

    println!("{}", overlaps_2);
}

fn read_input() -> Vec<((i32, i32), (i32, i32))> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    return io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let cap = re.captures(line.as_ref().unwrap()).unwrap();
            return (
                (
                    cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ),
                (
                    cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                ),
            );
        })
        .fold(Vec::<((i32, i32), (i32, i32))>::new(), |mut lines, line| {
            lines.push(line);
            return lines;
        });
}
