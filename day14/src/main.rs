use std::cmp;
use std::io;

const MIN_HORIZONTAL: usize = 50;
const SAND: (usize, usize) = (0, 450);

// const CAVE_BOUNDS: (usize, usize) = (100, 200);
const CAVE_BOUNDS: (usize, usize) = (900, 169);
// const CAVE_BOUNDS: (usize, usize) = (100, 11);

fn main() {
    let input = read_input()
        .iter()
        .map(|coords| {
            coords
                .iter()
                .map(|(x, y)| (x - MIN_HORIZONTAL, y - 0))
                .collect()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();

    let mut cave = [[0; CAVE_BOUNDS.0]; CAVE_BOUNDS.1];

    input.iter().for_each(|wall| {
        wall.windows(2).for_each(|points| {
            let (x1, y1) = points.first().unwrap().clone();
            let (x2, y2) = points.last().unwrap().clone();

            if x1 == x2 {
                for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                    cave[y][x1] = 1;
                }
            }
            if y1 == y2 {
                for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                    cave[y1][x] = 1;
                }
            }
        });
    });

    let mut sand_i = 0;
    loop {
        let mut c = SAND.clone();
        loop {
            match cave.get(c.0 + 1) {
                Some(row) => {
                    if row[c.1] == 0 {
                        c = (c.0 + 1, c.1);
                        continue;
                    }
                    if row[c.1 - 1] == 0 {
                        c = (c.0 + 1, c.1 - 1);
                        continue;
                    }
                    if row[c.1 + 1] == 0 {
                        c = (c.0 + 1, c.1 + 1);
                        continue;
                    }
                    break;
                }
                None => break,
            }
        }

        if cave[c.0][c.1] == 2 {
            break
        }

        cave[c.0][c.1] = 2;
        sand_i += 1;
    }

    print_cave(&cave);
    // println!("{}", sand_i);
}

fn read_input() -> Vec<Vec<(usize, usize)>> {
    io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    s.split_once(',')
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn print_cave(cave: &[[usize; CAVE_BOUNDS.0]; CAVE_BOUNDS.1]) {
    cave.iter().for_each(|line| {
        line.iter().for_each(|e| match e {
            0 => print!("."),
            1 => print!("#"),
            2 => print!("O"),
            _ => unreachable!(""),
        });
        println!();
    });
}
