use std::collections::{HashMap, HashSet};
use std::io;

use regex::Regex;

// const ROW: i32 = 10;
// const RANGE_1: i32 = 30;
const ROW: i32 = 2000000;
const RANGE_1: i32 = 8_000_000;
const RANGE_2: i32 = 4_000_000;

fn main() {
    let (sensors, beacons) = read_input();

    let sensor_to_dist = sensors
        .iter()
        .map(|sensor| (*sensor, dist_to_beacon(*sensor, &beacons)))
        .collect::<HashMap<(i32, i32), u32>>();

    let mut cnt = 0;
    for i in -RANGE_1..=RANGE_1 {
        if sensors
            .iter()
            .any(|sensor| dist_to((i, ROW), *sensor) <= sensor_to_dist[sensor])
            && !beacons.contains(&(i, ROW))
        {
            cnt += 1;
        }
    }
    println!("{}", cnt);

    for (sensor, dist) in sensor_to_dist.iter() {
        let edge = sensor_edge(*sensor, *dist);
        let f = edge.iter().find(|&field| {
            if field.0 < 0 || field.0 > RANGE_2 {
                return false;
            }
            if field.1 < 0 || field.1 > RANGE_2 {
                return false;
            }

            return !sensor_to_dist
                .iter()
                .any(|(s, &dist)| dist_to(*field, *s) <= dist)
                && !beacons.contains(&field);
        });

        if f.is_some() {
            let (x, y) = f.unwrap();
            println!(
                "{} * {} + {} = {}",
                x,
                RANGE_2,
                y,
                (*x as i64) * (RANGE_2 as i64) + (*y as i64)
            );
            break;
        }
    }
}

fn dist_to_beacon(sensor: (i32, i32), beacons: &Vec<(i32, i32)>) -> u32 {
    beacons
        .iter()
        .map(|&beacon| dist_to(sensor, beacon))
        .min()
        .unwrap()
}

fn dist_to(x: (i32, i32), y: (i32, i32)) -> u32 {
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}

fn sensor_edge(sensor: (i32, i32), dist: u32) -> HashSet<(i32, i32)> {
    let edge_dist = (dist + 1) as i32;

    (0..=(edge_dist))
        .flat_map(|i| {
            vec![
                (sensor.0 + edge_dist - i, sensor.1 - i),
                (sensor.0 + edge_dist - i, sensor.1 + i),
                (sensor.0 - edge_dist + i, sensor.1 - i),
                (sensor.0 - edge_dist + i, sensor.1 + i),
            ]
        })
        .collect()
}

fn read_input() -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let reg =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();

    let input = io::stdin()
        .lines()
        .flatten()
        .flat_map(|line| {
            reg.captures(&line)
                .iter()
                .map(|cap| {
                    let s_x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let s_y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let b_x = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    let b_y = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
                    ((s_x, s_y), (b_x, b_y))
                })
                .collect::<Vec<((i32, i32), (i32, i32))>>()
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    (
        input.iter().map(|(sensor, _)| *sensor).collect(),
        input.iter().map(|(_, beacon)| *beacon).collect(),
    )
}
