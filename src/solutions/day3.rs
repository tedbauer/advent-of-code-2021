use std::io::{self, BufRead};

pub fn part1() {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<String>>();
    let num_bits = input.get(0).unwrap().len();

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for bit in 0..num_bits {
        let one_count = input
            .iter()
            .map(|binary_num| binary_num.chars().nth(bit).unwrap())
            .filter(|b| *b == '1')
            .collect::<Vec<char>>()
            .len();
        if one_count > input.len() - one_count {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
    let esp_int = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("gamma: {}", gamma_int);
    println!("epsilon: {}", esp_int);
    println!("product: {}", gamma_int * esp_int);
}

pub fn part2() {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<String>>();
    let num_bits = input.get(0).unwrap().len();

    let mut nums = input.clone();
    for bit in 0..num_bits {
        let ones = nums
            .clone()
            .iter()
            .map(|n| n.chars().nth(bit).unwrap())
            .filter(|b| *b == '1')
            .count();
        let popular_bit = if ones >= nums.len() - ones { '1' } else { '0' };
        nums.retain(|n| n.chars().nth(bit).unwrap() == popular_bit);
				if nums.len() == 1 { break }
    }
    let oxy_rating = nums.get(0).unwrap();

    let mut nums = input.clone();
    for bit in 0..num_bits {
        let ones = nums
            .clone()
            .iter()
            .map(|n| n.chars().nth(bit).unwrap())
            .filter(|b| *b == '1')
            .count();
        let popular_bit = if ones < nums.len() - ones { '1' } else { '0' };
        nums.retain(|n| n.chars().nth(bit).unwrap() == popular_bit);
				if nums.len() == 1 { break }
    }
    let carbon_rating = nums.get(0).unwrap();

    let oxy_int = isize::from_str_radix(&oxy_rating, 2).unwrap();
    let carbon_int = isize::from_str_radix(&carbon_rating, 2).unwrap();

    println!("oxy: {}", oxy_int);
    println!("carbon: {}", carbon_int);
    println!("product: {}", oxy_int * carbon_int);
}

