use std::cmp;
use std::io::{self, BufRead};

fn cost(start: i64, target: i64) -> i64 {
    let mut cost = 0;
    let mut i = 1;
    let min = cmp::min(start, target);
    let max = cmp::max(start, target);
    for _ in min..max {
        cost += i;
        i += 1;
    }
    cost
}

pub fn part2() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();

    let mut timers = input
        .split(",")
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    timers.sort_unstable();

    let avg: i64 = ((timers.iter().sum::<i64>() as f64) / (timers.len() as f64)).round() as i64;
    let fuel_sum_avg = timers.iter().fold(0, |acc, n| acc + cost(*n, avg));

    let max = timers.iter().max().unwrap();

    let mut smallest = None;
    for i in 0..*max {
        let fuel_sum = timers.iter().fold(0, |acc, n| acc + cost(*n, i));
        println!("fuel sum for anchor {}: {}", i, fuel_sum);
        match smallest {
            None => smallest = Some(fuel_sum),
            Some(s) => {
                if fuel_sum < s {
                    smallest = Some(fuel_sum);
                }
            }
        }
    }

		println!("smallest is: {}", smallest.unwrap());

    println!("{:?}", timers);
    println!("avg: {}", avg);
    println!("fuel sum: {}", fuel_sum_avg);
}

pub fn part1() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();

    let mut timers = input
        .split(",")
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    timers.sort_unstable();

    let median = timers.get(timers.len() / 2).unwrap();
    let fuel_sum = timers.iter().fold(0, |acc, n| acc + (n - median).abs());

    println!("{:?}", timers);
    println!("median: {}", median);
    println!("fuel sum: {}", fuel_sum);
}
