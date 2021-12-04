use std::io::{self, BufRead};

pub fn part1() {
    let stdin = io::stdin();
    let mut hor_pos = 0;
    let mut depth = 0;
    for line in stdin.lock().lines() {
        let current = line.unwrap().trim().parse::<String>().unwrap();
        let mut split = current.split(" ");
        let dir = split.next().unwrap();
        let val = split.next().unwrap().trim().parse::<i64>().unwrap();

        match dir {
            "forward" => hor_pos += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => panic!("invalid cmd"),
        }
    }
    println!("hor: {}", hor_pos);
    println!("depth: {}", depth);
    println!("product: {}", hor_pos * depth);
}

pub fn part2() {
    let stdin = io::stdin();
    let mut hor_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in stdin.lock().lines() {
        let current = line.unwrap().trim().parse::<String>().unwrap();
        let mut split = current.split(" ");
        let dir = split.next().unwrap();
        let val = split.next().unwrap().trim().parse::<i64>().unwrap();

        match dir {
            "down" => aim += val,
            "up" => aim -= val,
            "forward" => {
                hor_pos += val;
                depth += aim * val;
            }
            _ => panic!("invalid cmd"),
        }
    }
    println!("hor: {}", hor_pos);
    println!("depth: {}", depth);
    println!("product: {}", hor_pos * depth);
}
