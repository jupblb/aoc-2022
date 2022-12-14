use std::cmp;
use std::{
    collections::{HashSet, VecDeque},
    io,
};

#[derive(Debug)]
struct Map {
    starts: Vec<(usize, usize)>,
    end: (usize, usize),
    heights: Vec<Vec<u32>>,
}

struct Step {
    coords: (usize, usize),
    steps: usize,
}

impl Step {
    fn next(&self, x: usize, y: usize) -> Step {
        return Step {
            coords: (x, y),
            steps: self.steps + 1,
        };
    }
}

fn main() {
    let map = read_input();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<Step> = VecDeque::new();

    for start in map.starts {
        println!("Push {:?}", start);
        queue.push_back(Step {
            coords: start,
            steps: 0,
        });
    }

    while !queue.is_empty() {
        let front = queue.pop_front().unwrap();
        let coords = front.coords;

        if visited.contains(&coords) {
            continue;
        }
        visited.insert(coords);

        if map.heights[coords.0][coords.1] == 'z'.to_digit(36).unwrap() + 1 {
            println!("{}", front.steps);
            break;
        }

        for i in [-1, 1] {
            for j in [-1, 1] {
                let i32_coords = (coords.0 as i32, coords.1 as i32);
                let new_coords_1 = (cmp::max(i32_coords.0 + i, 0) as usize, coords.1);
                let new_coords_2 = (coords.0, cmp::max(i32_coords.1 + j, 0) as usize);

                map.heights.get(new_coords_1.0).iter().for_each(|&row| {
                    row.get(new_coords_1.1).iter().for_each(|&el| {
                        if *el <= 1 + map.heights[coords.0][coords.1] {
                            queue.push_back(front.next(new_coords_1.0, new_coords_1.1));
                        }
                    });
                });
                map.heights.get(new_coords_2.0).iter().for_each(|&row| {
                    row.get(new_coords_2.1).iter().for_each(|&el| {
                        if *el <= 1 + map.heights[coords.0][coords.1] {
                            queue.push_back(front.next(new_coords_2.0, new_coords_2.1));
                        }
                    });
                });
            }
        }
    }
}

fn read_input() -> Map {
    let mut map = Map {
        starts: Vec::new(),
        end: (0, 0),
        heights: Vec::new(),
    };

    io::stdin()
        .lines()
        .flatten()
        .enumerate()
        .for_each(|(i, line)| {
            let chars = line
                .chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' | 'a' => {
                        map.starts.push((i, j));
                        'a'.to_digit(36).unwrap()
                    }
                    'E' => {
                        map.end = (i, j);
                        'z'.to_digit(36).unwrap() + 1
                    }
                    _ => c.to_digit(36).unwrap(),
                })
                .collect();

            map.heights.push(chars);
        });

    map.starts
        .sort_by_key(|(x, y)| x.abs_diff(map.end.0) + y.abs_diff(map.end.1));

    return map;
}
