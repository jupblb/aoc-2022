use std::collections::BinaryHeap;

fn main() {
    let mut calories = BinaryHeap::<i32>::new();

    include_str!("../input.in").lines().fold(0, |c_sum, c| {
        if c == "" {
            calories.push(c_sum);
            return 0;
        }
        return c_sum + c.parse::<i32>().unwrap();
    });

    let c1 = calories.pop().unwrap();
    let c2 = calories.pop().unwrap();
    let c3 = calories.pop().unwrap();

    println!("{}", c1);
    println!("{}", c1 + c2 + c3);
}
