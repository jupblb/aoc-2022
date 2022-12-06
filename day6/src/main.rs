use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let input = read_input();

    solve(&input, 4);
    println!("---");
    solve(&input, 14);
}

fn solve(input: &Vec<String>, w: usize) {
    input.iter().for_each(|line| {
        let enumerated_line = line
            .as_str()
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>();

        for window in enumerated_line.windows(w) {
            let set = HashSet::<char>::from_iter(window.iter().map(|c| c.1));

            if set.len() == w {
                window
                    .last()
                    .iter()
                    .for_each(|(i, _)| println!("{}", i + 1));
                break;
            }
        }
    });
}

fn read_input() -> Vec<String> {
    return io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
}
