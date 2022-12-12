use std::io;

use regex::Regex;

#[derive(Debug)]
enum Op {
    ADD(usize),
    MULT(usize),
    SQUARE,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Op,
    test: usize,
    test_true: usize,
    test_false: usize,
    inspections: usize,
}

fn main() {
    let mut monkeys = read_input();
    let test_mult = monkeys.iter().map(|m| m.test).fold(1, |acc, t| acc * t);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let new_items = Vec::new();
            let old_items = monkeys[i].items.clone();

            old_items.iter().for_each(|item| {
                let mut new_item = match monkeys[i].operation {
                    Op::ADD(n) => item + n,
                    Op::MULT(n) => item * n,
                    Op::SQUARE => item * item,
                };

                // new_item /= 3;
                new_item %= test_mult;

                if new_item % monkeys[i].test == 0 {
                    let monkey = monkeys[i].test_true;
                    monkeys[monkey].items.push(new_item);
                } else {
                    let monkey = monkeys[i].test_false;
                    monkeys[monkey].items.push(new_item);
                }

                monkeys[i].inspections += 1;
            });

            monkeys[i].items = new_items;
        }
    }

    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();

    println!("{}", monkeys[0].inspections * monkeys[1].inspections);
}

fn read_input() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let monkey_reg = Regex::new(r"^Monkey \d:$").unwrap();
    let items_reg = Regex::new(r"^  Starting items: (.+)$").unwrap();
    let op_add_reg = Regex::new(r"^  Operation: new = old \+ (\d+)$").unwrap();
    let op_mult_reg = Regex::new(r"^  Operation: new = old \* (\d+)$").unwrap();
    let op_square_reg = Regex::new(r"^  Operation: new = old \* old$").unwrap();
    let test_reg = Regex::new(r"^  Test: divisible by (\d+)$").unwrap();
    let test_true_reg = Regex::new(r"^    If true: throw to monkey (\d)$").unwrap();
    let test_false_reg = Regex::new(r"^    If false: throw to monkey (\d)$").unwrap();

    io::stdin().lines().flatten().for_each(|line| {
        if monkey_reg.is_match(&line) {
            monkeys.push(Monkey {
                items: Vec::new(),
                operation: Op::ADD(0),
                test: 0,
                test_true: 0,
                test_false: 0,
                inspections: 0,
            });
            return;
        }

        let id = monkeys.len() - 1;

        items_reg.captures(&line).iter().for_each(|cap| {
            let items: Vec<usize> = cap
                .get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|el| el.trim().parse::<usize>().unwrap())
                .collect();
            monkeys[id].items = items;
        });

        op_add_reg.captures(&line).iter().for_each(|cap| {
            let add = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            monkeys[id].operation = Op::ADD(add);
        });

        op_mult_reg.captures(&line).iter().for_each(|cap| {
            let add = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            monkeys[id].operation = Op::MULT(add);
        });

        op_square_reg.captures(&line).iter().for_each(|_| {
            monkeys[id].operation = Op::SQUARE;
        });

        test_reg.captures(&line).iter().for_each(|cap| {
            let test = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            monkeys[id].test = test;
        });

        test_true_reg.captures(&line).iter().for_each(|cap| {
            let test = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            monkeys[id].test_true = test;
        });

        test_false_reg.captures(&line).iter().for_each(|cap| {
            let test = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            monkeys[id].test_false = test;
        });
    });
    return monkeys;
}
