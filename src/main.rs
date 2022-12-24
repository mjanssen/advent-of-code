pub mod days;
pub mod lib;

use regex::Regex;
use std::{collections::HashMap, env};

use crate::days::ExecuteResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Supply a day argument. ie. `cargo run day1`".into());
    }

    let day = &args[1];

    let day_check = Regex::new(r"^day\d{1,2}$").unwrap();

    if day_check.is_match(day) == false {
        return Err("Supplied day was not correct, use 'day1' for example".into());
    }

    let mut methods: HashMap<String, fn() -> ExecuteResponse> = HashMap::new();

    methods.insert(String::from("day1"), days::day_1::execute);
    methods.insert(String::from("day2"), days::day_2::execute);
    methods.insert(String::from("day3"), days::day_3::execute);
    methods.insert(String::from("day4"), days::day_4::execute);
    methods.insert(String::from("day5"), days::day_5::execute);
    methods.insert(String::from("day6"), days::day_6::execute);
    methods.insert(String::from("day7"), days::day_7::execute);
    methods.insert(String::from("day8"), days::day_8::execute);
    methods.insert(String::from("day9"), days::day_9::execute);
    methods.insert(String::from("day10"), days::day_10::execute);
    methods.insert(String::from("day11"), days::day_11::execute);
    methods.insert(String::from("day12"), days::day_12::execute);
    methods.insert(String::from("day13"), days::day_13::execute);
    methods.insert(String::from("day14"), days::day_14::execute);
    methods.insert(String::from("day18"), days::day_18::execute);

    let method = match methods.get(day) {
        Some(function) => function,
        _ => return Err(format!("{} has not been implemented", day).into()),
    };

    println!("Results for {}", day);

    method()?;

    Ok(())
}
