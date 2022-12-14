use std::{cmp::Ordering, io};

use serde_json::{json, Value};

fn main() {
    let input = read_input();

    let score_1: usize = input
        .iter()
        .enumerate()
        .map(|(i, (v1, v2))| {
            if compare(v1, v2) == Ordering::Less {
                return i + 1;
            }
            return 0;
        })
        .sum();

    println!("{}", score_1);

    let mut new_input: Vec<Value> = input
        .iter()
        .flat_map(|(v1, v2)| vec![v1.clone(), v2.clone()])
        .collect();
    new_input.push(json!([[2]]));
    new_input.push(json!([[6]]));

    new_input.sort_by(compare);

    new_input.iter().enumerate().for_each(|(i, v)| {
        let v_str = v.to_string();
        if v_str == "[[2]]" || v_str == "[[6]]" {
            println!("{}", i + 1);
        }
    });
}

fn read_input() -> Vec<(Value, Value)> {
    let lines: Vec<String> = io::stdin()
        .lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .collect();

    lines
        .chunks(2)
        .map(|lines| {
            (
                serde_json::from_str(&lines[0]).unwrap(),
                serde_json::from_str(&lines[1]).unwrap(),
            )
        })
        .collect()
}

fn compare(v1: &Value, v2: &Value) -> Ordering {
    match (v1, v2) {
        (Value::Array(v1), Value::Array(v2)) => {
            for i in 0..v1.len().max(v2.len()) {
                match (v1.get(i), v2.get(i)) {
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(x), Some(y)) => match compare(x, y) {
                        Ordering::Equal => {}
                        o => return o,
                    },
                }
            }
            Ordering::Equal
        }
        (Value::Array(_), Value::Number(_)) => compare(v1, &Value::Array(vec![v2.clone()])),
        (Value::Number(x), Value::Number(y)) => x.as_i64().unwrap().cmp(&y.as_i64().unwrap()),
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![v1.clone()]), v2),
        _ => Ordering::Equal,
    }
}
