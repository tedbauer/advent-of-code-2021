extern crate argparse;
use argparse::{ArgumentParser, Store};

use std::io::{self, BufRead};

// TODO: move to a submodule
fn day1_part1() {
    let stdin = io::stdin();
    let mut increases = 0;
    let mut previous = None;
    for line in stdin.lock().lines() {
        let current = line.unwrap().trim().parse::<i64>().unwrap();
        if let Some(p) = previous {
            if current > p {
                increases += 1;
            }
        }
        previous = Some(current);
    }
    println!("{}", increases);
}

fn day1_part2() {
    let stdin = io::stdin();
    let mut increases = 0;
    let mut prev_sum = None;
    for window in stdin
        .lock()
        .lines()
        .map(|n| n.unwrap().trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .windows(3)
    {
        let sum = window[0] + window[1] + window[2];
        if let Some(p) = prev_sum {
            if sum > p {
                increases += 1;
            }
        }
        prev_sum = Some(sum);
    }
    println!("{}", increases);
}

fn day2_part1() {
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

fn day2_part2() {
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

fn main() {
    let mut name = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Advent of code 2021!");
        ap.refer(&mut name)
            .add_option(&["--name"], Store, "day/part");
        ap.parse_args_or_exit();
    }

    match name.as_str() {
        "day1part1" => day1_part1(),
        "day1part2" => day1_part2(),
        "day2part1" => day2_part1(),
        "day2part2" => day2_part2(),
        _ => println!("nothing with that name"),
    }
}
