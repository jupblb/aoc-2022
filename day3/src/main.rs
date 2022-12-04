use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let input = read_input();

    let priority_sum_1: u32 = input
        .iter()
        .map(|line| {
            let line_chars = line.chars().collect::<Vec<char>>();
            let (first_compartment, second_compartment) = line_chars.split_at(line_chars.len() / 2);
            let first_compartment_set = HashSet::<char>::from_iter(first_compartment.to_vec());
            let second_compartment_set = HashSet::<char>::from_iter(second_compartment.to_vec());

            let priority_sum: u32 = second_compartment_set
                .iter()
                .map(|c| {
                    if first_compartment_set.contains(c) {
                        return item_value(*c);
                    }
                    return 0;
                })
                .sum();

            return priority_sum;
        })
        .sum();

    println!("{}", priority_sum_1);

    let priority_sum_2: u32 = input
        .chunks(3)
        .map(|group| {
            let mut items = HashMap::<char, i32>::new();

            for rucksack in group {
                for c in rucksack.chars().collect::<HashSet<char>>() {
                    let item_count = items.get(&c).unwrap_or(&0);
                    items.insert(c, *item_count + 1);
                }
            }

            return items.iter().fold(0, |acc, (c, i)| {
                if *i == 3 {
                    return acc + item_value(*c);
                }
                return acc;
            });
        })
        .sum();

    println!("{}", priority_sum_2);
}

fn read_input() -> Vec<String> {
    return io::stdin()
        .lock()
        .lines()
        .fold(Vec::<String>::new(), |mut lines, line| {
            lines.push(line.unwrap());
            return lines;
        });
}

fn item_value(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - ('A' as u32) + 1 + 26;
    }
    return c as u32 - ('a' as u32) + 1;
}
