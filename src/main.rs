extern crate argparse;
use argparse::{ArgumentParser, Store};

use std::io::{self, BufRead};

// Day 1
// TODO: move to a submodule
fn part1() {
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

fn part2() {
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
        "day1part1" => part1(),
        "day1part2" => part2(),
        _ => println!("nothing with that name")
    }
}
