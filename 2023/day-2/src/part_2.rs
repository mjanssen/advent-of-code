use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Colors {
    Red,
    Green,
    Blue,
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Colors::Red => write!(f, "red"),
            Colors::Green => write!(f, "green"),
            Colors::Blue => write!(f, "blue"),
        }
    }
}

fn extract_numbers(string: &str) -> i32 {
    let f = string.chars().fold("".to_string(), |mut s, c| {
        match c.to_digit(10) {
            Some(_) => {
                s += &c.to_string();
                ()
            }
            None => (),
        }
        s
    });

    match f.parse::<i32>() {
        Ok(n) => n,
        _ => 0,
    }
}

const COLORS: [Colors; 3] = [Colors::Red, Colors::Green, Colors::Blue];

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let answer = _input.lines().fold(0, |mut count, line| {
        let mut game_minimum_set = HashMap::new();
        game_minimum_set.insert(Colors::Red, 1);
        game_minimum_set.insert(Colors::Green, 1);
        game_minimum_set.insert(Colors::Blue, 1);

        let mut power = 1;

        let games = line.split(":").last().unwrap().trim();
        for game in games.split(";") {
            for round in game.split(",") {
                let number = extract_numbers(round);

                for color in COLORS {
                    if round.contains(&color.to_string()) {
                        if &number > game_minimum_set.get(&color).unwrap() {
                            game_minimum_set.insert(color, number);
                        }
                    }
                }
            }
        }

        for (_k, v) in game_minimum_set {
            power *= v;
        }

        count += power;

        count
    });

    Ok(answer.to_string())
}
