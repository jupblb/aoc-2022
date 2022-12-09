use regex::Regex;
use std::collections::HashSet;
use std::io;

const TAIL_LENGTH: usize = 9;

#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let mut input = read_input();
    for i in 0..TAIL_LENGTH {
        // for dir in &input {
        //     match dir {
        //         Direction::UP => println!("UP"),
        //         Direction::DOWN => println!("DOWN"),
        //         Direction::LEFT => println!("LEFT"),
        //         Direction::RIGHT => println!("RIGHT"),
        //     }
        // }
        let (cnt, dirs) = solve(&input);
        println!("{}: {}", i, cnt);
        input = dirs;
        println!("----------------------");
    }
}

fn solve(dirs: &Vec<Direction>) -> (usize, Vec<Direction>) {
    let mut next_dirs = Vec::new();
    let mut coords_h: HashSet<(i32, i32)> = HashSet::new();
    let mut coords_t: HashSet<(i32, i32)> = HashSet::new();
    let (mut h_x, mut h_y): (i32, i32) = (0, 0);
    let (mut t_x, mut t_y): (i32, i32) = (0, 0);
    coords_h.insert((0, 0));
    coords_t.insert((0, 0));

    dirs.iter().for_each(|direction| match direction {
        Direction::UP => {
            if dist_more_than_1(h_x - 1, h_y, t_x, t_y) {
                if next_dirs.last().unwrap_or(&Direction::LEFT) != &Direction::UP {
                    next_dirs.push(Direction::UP);
                    if t_y < h_y {
                        next_dirs.push(Direction::RIGHT);
                    }
                    if t_y > h_y {
                        next_dirs.push(Direction::LEFT);
                    }
                } else {
                    if t_y < h_y {
                        next_dirs.push(Direction::RIGHT);
                    }
                    if t_y > h_y {
                        next_dirs.push(Direction::LEFT);
                    }
                    next_dirs.push(Direction::UP);
                }
                (t_x, t_y) = (h_x, h_y);
                coords_t.insert((t_x, t_y));
            }
            h_x -= 1;
            coords_h.insert((h_x, h_y));
        }
        Direction::DOWN => {
            if dist_more_than_1(h_x + 1, h_y, t_x, t_y) {
                if next_dirs.last().unwrap_or(&Direction::LEFT) != &Direction::DOWN {
                    next_dirs.push(Direction::DOWN);
                    if t_y < h_y {
                        next_dirs.push(Direction::RIGHT);
                    }
                    if t_y > h_y {
                        next_dirs.push(Direction::LEFT);
                    }
                } else {
                    if t_y < h_y {
                        next_dirs.push(Direction::RIGHT);
                    }
                    if t_y > h_y {
                        next_dirs.push(Direction::LEFT);
                    }
                    next_dirs.push(Direction::DOWN);
                }
                (t_x, t_y) = (h_x, h_y);
                coords_t.insert((t_x, t_y));
            }
            h_x += 1;
            coords_h.insert((h_x, h_y));
        }
        Direction::LEFT => {
            if dist_more_than_1(h_x, h_y - 1, t_x, t_y) {
                if next_dirs.last().unwrap_or(&Direction::UP) != &Direction::LEFT {
                    next_dirs.push(Direction::LEFT);
                    if t_x < h_x {
                        next_dirs.push(Direction::DOWN);
                    }
                    if t_x > h_x {
                        next_dirs.push(Direction::UP);
                    }
                } else {
                    if t_x < h_x {
                        next_dirs.push(Direction::DOWN);
                    }
                    if t_x > h_x {
                        next_dirs.push(Direction::UP);
                    }
                    next_dirs.push(Direction::LEFT);
                }
                (t_x, t_y) = (h_x, h_y);
                coords_t.insert((t_x, t_y));
            }
            h_y -= 1;
            coords_h.insert((h_x, h_y));
        }
        Direction::RIGHT => {
            if dist_more_than_1(h_x, h_y + 1, t_x, t_y) {
                if next_dirs.last().unwrap_or(&Direction::UP) != &Direction::RIGHT {
                    next_dirs.push(Direction::RIGHT);
                    if t_x < h_x {
                        next_dirs.push(Direction::DOWN);
                    }
                    if t_x > h_x {
                        next_dirs.push(Direction::UP);
                    }
                } else {
                    if t_x < h_x {
                        next_dirs.push(Direction::DOWN);
                    }
                    if t_x > h_x {
                        next_dirs.push(Direction::UP);
                    }
                    next_dirs.push(Direction::RIGHT);
                }
                (t_x, t_y) = (h_x, h_y);
                coords_t.insert((t_x, t_y));
            }
            h_y += 1;
            coords_h.insert((h_x, h_y));
        }
    });

    return (coords_t.len(), next_dirs);
}

fn dist_more_than_1(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    if x1.abs_diff(x2) > 1 {
        return true;
    }
    if y1.abs_diff(y2) > 1 {
        return true;
    }
    return false;
}

fn read_input() -> Vec<Direction> {
    let reg = Regex::new(r"^([RULD]) (\d+)$").unwrap();
    return io::stdin()
        .lines()
        .flatten()
        .flat_map(|line| {
            return reg.captures(&line).map(|cap| {
                let dir = cap.get(1).unwrap().as_str().chars().next().unwrap();
                let steps = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();

                match dir {
                    'U' => vec![Direction::UP; steps],
                    'D' => vec![Direction::DOWN; steps],
                    'L' => vec![Direction::LEFT; steps],
                    'R' => vec![Direction::RIGHT; steps],
                    _ => Vec::new(),
                }
            });
        })
        .flatten()
        .collect();
}
