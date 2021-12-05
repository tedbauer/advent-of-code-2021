use std::cmp;
use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn part2() {
    let stdin = io::stdin();

    let mut inp = stdin.lock().lines().map(|r| r.unwrap());

    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();

    loop {
        match inp.next() {
            Some(line) => {
                let mut points_split = line.split("->");
                let mut point1_split = points_split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.trim().parse::<i64>().unwrap());
                let mut point2_split = points_split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.trim().parse::<i64>().unwrap());

                let point1 = (point1_split.next(), point1_split.next());
                let point2 = (point2_split.next(), point2_split.next());

                match (point1, point2) {
                    ((Some(x0), Some(y0)), (Some(x1), Some(y1))) => {
                        if x0 == x1 || y0 == y1 {
                            let x_lo = cmp::min(x0, x1);
                            let x_hi = cmp::max(x0, x1);
                            let y_lo = cmp::min(y0, y1);
                            let y_hi = cmp::max(y0, y1);
                            for x in x_lo..(x_hi + 1) {
                                for y in y_lo..(y_hi + 1) {
                                    match counts.get_mut(&(x as u32, y as u32)) {
                                        Some(c) => *c += 1,
                                        None => {
                                            counts.insert((x as u32, y as u32), 1);
                                            ()
                                        }
                                    }
                                }
                            }
                        } else {
                            let x_sign = if x0 < x1 { 1 } else { -1 };
                            let y_sign = if y0 < y1 { 1 } else { -1 };
                            let mut i = 0;
                            loop {
                                let x = x0 + i * x_sign;
                                let y = y0 + i * y_sign;
                                match counts.get_mut(&(x as u32, y as u32)) {
                                    Some(c) => *c += 1,
                                    None => {
                                        counts.insert((x as u32, y as u32), 1);
                                        ()
                                    }
                                }
                                i += 1;
                                if x == x1 && y == y1 {
                                    break;
                                }
                            }
                        }
                    }
                    _ => panic!("points are bad"),
                }
            }
            None => break,
        }
    }
    println!("count: {}", counts.iter().filter(|(_, v)| **v >= 2).count());
}

pub fn part1() {
    let stdin = io::stdin();

    let mut inp = stdin.lock().lines().map(|r| r.unwrap());

    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();

    loop {
        match inp.next() {
            Some(line) => {
                let mut points_split = line.split("->");
                let mut point1_split = points_split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.trim().parse::<i64>().unwrap());
                let mut point2_split = points_split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.trim().parse::<i64>().unwrap());

                let point1 = (point1_split.next(), point1_split.next());
                let point2 = (point2_split.next(), point2_split.next());

                match (point1, point2) {
                    ((Some(x0), Some(y0)), (Some(x1), Some(y1))) => {
                        if x0 == x1 || y0 == y1 {
                            let x_lo = cmp::min(x0, x1);
                            let x_hi = cmp::max(x0, x1);
                            let y_lo = cmp::min(y0, y1);
                            let y_hi = cmp::max(y0, y1);
                            for x in x_lo..(x_hi + 1) {
                                for y in y_lo..(y_hi + 1) {
                                    match counts.get_mut(&(x as u32, y as u32)) {
                                        Some(c) => *c += 1,
                                        None => {
                                            counts.insert((x as u32, y as u32), 1);
                                            ()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => panic!("points are bad"),
                }
            }
            None => break,
        }
    }
    println!("count: {}", counts.iter().filter(|(_, v)| **v >= 2).count());
}
