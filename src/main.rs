extern crate argparse;
use argparse::{ArgumentParser, Store};

mod solutions;
use solutions::{day1, day2, day3, day4, day5, day6};

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
        "day1part1" => day1::part1(),
        "day1part2" => day1::part2(),
        "day2part1" => day2::part1(),
        "day2part2" => day2::part2(),
        "day3part1" => day3::part1(),
        "day3part2" => day3::part2(),
        "day4part1" => day4::part1(),
        "day4part2" => day4::part2(),
        "day5part1" => day5::part1(),
        "day5part2" => day5::part2(),
        "day6part1" => day6::part1(),
        "day6part2" => day6::part2(),
        _ => println!("nothing with that name"),
    }
}
