pub mod days;
pub mod lib;

use regex::Regex;
use std::{collections::HashMap, env};

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

    let mut methods: HashMap<String, fn() -> Result<(), Box<dyn std::error::Error>>> =
        HashMap::new();

    methods.insert(String::from("day1"), days::day_1::execute);

    let method = match methods.get(day) {
        Some(function) => function,
        _ => return Err(format!("{} has not been implemented", day).into()),
    };

    println!("Results for {}", day);

    method()?;

    Ok(())
}
