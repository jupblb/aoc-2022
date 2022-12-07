use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, BufRead},
    rc::Rc,
};

use regex::Regex;

struct Node {
    size: usize,

    parent: Option<Rc<RefCell<Node>>>,
    children: HashMap<String, Rc<RefCell<Node>>>,
}

fn main() {
    let mut node = Rc::new(RefCell::new(Node {
        size: 0,
        parent: None,
        children: HashMap::new(),
    }));

    let cd = Regex::new(r"^\$ cd (.+)$").unwrap();
    let ls = Regex::new(r"^\$ ls$").unwrap();
    let dir_entry = Regex::new(r"^dir (.+)$").unwrap();
    let file_entry = Regex::new(r"^(\d+) (.+)$").unwrap();

    io::stdin()
        .lock()
        .lines()
        .skip(1)
        .flatten()
        .for_each(|line| {
            if ls.is_match(&line) {
                return;
            }

            cd.captures(&line).iter().for_each(|cap| {
                let dir = cap.get(1).unwrap().as_str();
                match dir {
                    "." => {}
                    ".." => node = node.clone().borrow().parent.as_ref().unwrap().clone(),
                    _ => node = node.clone().borrow().children.get(dir).unwrap().clone(),
                }
            });

            dir_entry.captures(&line).iter().for_each(|cap| {
                let name = cap.get(1).unwrap().as_str();
                node.borrow_mut().children.insert(
                    name.to_string(),
                    Rc::new(RefCell::new(Node {
                        size: 0,
                        parent: Some(node.clone()),
                        children: HashMap::new(),
                    })),
                );
            });

            file_entry.captures(&line).iter().for_each(|cap| {
                let size = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let name = cap.get(2).unwrap().as_str();

                node.borrow_mut().children.insert(
                    name.to_string(),
                    Rc::new(RefCell::new(Node {
                        size,
                        parent: Some(node.clone()),
                        children: HashMap::new(),
                    })),
                );
            });
        });

    while node.borrow().parent.is_some() {
        node = node.clone().borrow().parent.as_ref().unwrap().clone();
    }

    let (sizes, _) = count_size(node);

    let size_1 = sizes
        .iter()
        .filter(|&&s| s <= 100000)
        .fold(0, |acc, &s| acc + s);
    println!("{}", size_1);

    let total_size = sizes.last().unwrap();
    let free_size = 70000000 - total_size;
    let to_free = 30000000 - free_size;

    let size_2 = sizes
        .iter()
        .filter(|&&s| s >= to_free)
        .map(|s| *s)
        .min()
        .unwrap();
    println!("{}", size_2);
}

fn count_size(node: Rc<RefCell<Node>>) -> (Vec<usize>, usize) {
    let mut size = 0;
    let mut sizes: Vec<usize> = Vec::new();

    for child in node.borrow().children.values() {
        let (mut child_sizes, file_size) = count_size(child.clone());
        size += child_sizes.last().unwrap();
        size += file_size;
        sizes.append(&mut child_sizes);
    }

    let node_size = node.borrow().size;
    if node_size != 0 {
        sizes.push(0);
    } else {
        sizes.push(size + node.borrow().size);
    }

    return (sizes, node_size);
}
