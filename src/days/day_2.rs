// A Rock       Y Paper
// B Paper      X Rock
// C Scissors   Z Scissors

use crate::lib::load_file::load_data_file;

fn get_bet_score(bet: &str) -> u32 {
    match bet {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn get_round_score(opponent_bet: &str, bet: &str) -> u32 {
    let score: u32 = match opponent_bet {
        "A" => match bet {
            "Y" => 6,
            "X" => 3,
            _ => 0,
        },
        "B" => match bet {
            "Z" => 6,
            "Y" => 3,
            _ => 0,
        },
        "C" => match bet {
            "X" => 6,
            "Z" => 3,
            _ => 0,
        },
        _ => 0,
    };

    score + get_bet_score(bet)
}

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_file("day_2.txt")?;
    let scores: Vec<u32> = data
        .lines()
        .map(|line: &str| {
            let line_data = line.split(" ").collect::<Vec<&str>>();
            get_round_score(line_data[0], line_data[1])
        })
        .collect();

    let total_score: u32 = scores.iter().sum();

    println!("part 1 - {:?}", total_score);
    part_2(data);

    Ok(())
}

// A Rock       X Lose
// B Paper      Y Draw
// C Scissors   Z Win

fn p2_get_bet_score(bet: &str) -> u32 {
    match bet {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    }
}

fn p2_get_score(opponent_bet: &str, outcome: &str) -> u32 {
    let own_throw: Option<&str> = match opponent_bet {
        "A" => match outcome {
            "X" => Some("C"),
            "Z" => Some("B"),
            _ => Some("A"),
        },
        "B" => match outcome {
            "X" => Some("A"),
            "Z" => Some("C"),
            _ => Some("B"),
        },
        "C" => match outcome {
            "X" => Some("B"),
            "Z" => Some("A"),
            _ => Some("C"),
        },
        _ => None,
    };

    if let Some(throw) = own_throw {
        let score = match outcome {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        };

        return score + p2_get_bet_score(throw);
    }

    return 0;
}

fn part_2(data: String) -> () {
    let scores: Vec<u32> = data
        .lines()
        .map(|line: &str| {
            let line_data = line.split(" ").collect::<Vec<&str>>();
            p2_get_score(line_data[0], line_data[1])
        })
        .collect();

    let total_score: u32 = scores.iter().sum();

    println!("part 2 - {:?}", total_score);
}
