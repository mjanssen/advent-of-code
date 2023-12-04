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
    let mut rules: HashMap<Colors, i32> = HashMap::new();
    rules.insert(Colors::Red, 12);
    rules.insert(Colors::Green, 13);
    rules.insert(Colors::Blue, 14);

    let answer = _input.lines().enumerate().fold(0, |mut count, (i, line)| {
        let mut valid_game: bool = true;

        let games = line.split(":").last().unwrap().trim();
        'outer: for game in games.split(";") {
            for round in game.split(",") {
                let number = extract_numbers(round);

                for color in COLORS {
                    if round.contains(&color.to_string()) {
                        if &number > rules.get(&color).unwrap() {
                            valid_game = false;
                            break 'outer;
                        }
                    }
                }
            }
        }

        if valid_game {
            count += i + 1;
        }

        count
    });

    Ok(answer.to_string())
}
