use std::collections::HashMap;
use std::io::{self, BufRead};

type Memo = HashMap<(i64, i64), i64>;

fn fish_generated(timer: i64, days: i64, memo: Memo) -> (i64, Memo) {
    match memo.get(&(timer, days)) {
        Some(val) => (*val, memo),
        None => {
            if days - timer >= 0 {
                let (fish1, memo1) = fish_generated(7, days - timer, memo);
                let (fish2, mut memo2) = fish_generated(9, days - timer, memo1);
                let res = 1 + fish1 + fish2;
                memo2.insert((timer, days), res);
                (res, memo2)
            } else {
                (0, memo)
            }
        }
    }
}

pub fn part2() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();

    let mut timers = input
        .split(",")
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    const DAYS: i64 = 255;

    let mut fish_sum = 0;
    for timer in &timers {
        let memo: Memo = HashMap::new();
        let (fish, memo) = fish_generated(*timer, DAYS, memo);
				fish_sum += fish;
    }

    println!("fish sum: {}", fish_sum + timers.len() as i64);
}

pub fn part1() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();

    let mut timers = input
        .split(",")
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let days = 80;

    for _ in 0..days {
        let num_zeros = timers.iter().filter(|t| **t == 0).count();
        for timer in timers.iter_mut() {
            if *timer == 0 {
                *timer = 6;
            } else {
                *timer -= 1;
            }
        }
        timers.extend(&vec![8; num_zeros]);
    }

    println!("fish: {}", timers.len());
}
