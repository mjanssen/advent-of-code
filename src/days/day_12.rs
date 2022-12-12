use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    height: usize,
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_12.txt")?;

    let (start, board) = build_board(&data);

    // Possible steps for all blocks > index = step_number
    let mut possible_steps: Vec<Vec<Position>> = vec![];

    let mut steps: Vec<Position> = vec![start];
    let mut blocked_steps: Vec<Position> = vec![];

    let mut step_counter = 0;

    let round_amount = 1;

    for round in 0..round_amount {
        let mut found = false;

        // for _ in 0..10 {
        //     let step = get_disired_next_step(&board, &mut steps, &mut blocked_steps);
        //     if steps.contains(&step) == false {
        //         steps.push(step);
        //     }

        //     if step.height == 26 {
        //         found = true;
        //     }
        // }

        while found == false {
            let step =
                get_disired_next_step(&board, &mut steps, &mut blocked_steps, &mut possible_steps);
            if steps.contains(&step) == false {
                steps.push(step);
            }

            if step.height == 26 {
                found = true;
            }
        }

        let s = steps.len() - 1;

        if step_counter == 0 || s < step_counter {
            step_counter = s;
        }
    }

    board.iter().for_each(|horizontal_positions| {
        horizontal_positions.iter().for_each(|position| {
            if steps.contains(position) {
                print!("X");
            } else if blocked_steps.contains(position) {
                print!(".")
            } else {
                print!(".")
            }
        });

        println!("");
    });

    println!("Reached E in {} steps", step_counter);

    Ok(())
}

fn get_step_for_direction(
    current_step: &Position,
    steps: &Vec<Position>,
    blocked_steps: &Vec<Position>,
    step_to: Position,
) -> Option<(Position, i8)> {
    let contains = steps.contains(&step_to);
    let contains_blocked = blocked_steps.contains(&step_to);

    let mut result: Option<(Position, i8)> = None;

    if step_to.height <= current_step.height + 1 && contains == false && contains_blocked == false {
        // let direction_index = direction_order.find(direction).unwrap() as i8 - 2;
        let weight = (step_to.height as i8 - current_step.height as i8).signum() * 5;
        result = Some((step_to, weight));
        // available_steps.push(Some((step_to, 0 + weight)));
    }

    result
}

fn get_disired_next_step(
    board: &Vec<Vec<Position>>,
    steps: &mut Vec<Position>,
    blocked_steps: &mut Vec<Position>,
    possible_steps: &mut Vec<Vec<Position>>,
) -> Position {
    let current_step = steps.last().expect("Expected at least one step...");

    // Randomize disired direction / Reverse to calc the desired step by index
    let direction_order = get_direction_order().chars().rev().collect::<String>();
    // let direction_order = String::from("ULDR");

    if current_step.height == 26 {
        return *current_step;
    }

    // Steps are (Position, weight)
    let mut available_steps: Vec<Option<(Position, i8)>> = vec![];

    // DIRECTION UP
    if current_step.y == 0 {
        available_steps.push(None);
    } else {
        available_steps.push(get_step_for_direction(
            &current_step,
            &steps,
            &blocked_steps,
            board[current_step.y - 1][current_step.x],
        ));
    }

    // DIRECTION RIGHT
    if current_step.x >= board[0].len() - 1 {
        available_steps.push(None);
    } else {
        available_steps.push(get_step_for_direction(
            &current_step,
            &steps,
            &blocked_steps,
            board[current_step.y][current_step.x + 1],
        ));
    }

    // DIRECTION DOWN
    if current_step.y >= board.len() - 1 {
        available_steps.push(None);
    } else {
        available_steps.push(get_step_for_direction(
            &current_step,
            &steps,
            &blocked_steps,
            board[current_step.y + 1][current_step.x],
        ));
    }

    // DIRECTION LEFT
    if current_step.x == 0 {
        available_steps.push(None);
    } else {
        available_steps.push(get_step_for_direction(
            &current_step,
            &steps,
            &blocked_steps,
            board[current_step.y][current_step.x - 1],
        ));
    }

    // Prefered step up
    let mut step_up: Option<(Position, i8)> = None;

    available_steps.iter().for_each(|step_option| {
        if let Some(step) = step_option {
            match step_up {
                None => step_up = Some(*step),
                Some((p, weight)) => {
                    // Weighted steps
                    if step.1 > weight {
                        step_up = Some(*step);
                    }

                    // Height steps
                    // if step.0.height > p.height {
                    //     step_up = Some(*step);
                    // }
                }
            }
        }
    });

    // If a next step is avaialable, step
    if let Some(next_step) = step_up {
        // Check if we already could've jumped to this spot before
        // let p_s = possible_steps
        //     .iter()
        //     .position(|steps_vector| steps_vector.contains(&next_step.0));

        // if let Some(previous) = p_s {
        //     // println!("{:?}", next_step);
        //     // println!("{:?}", possible_steps[previous]);
        //     // println!("{:?}", previous);

        //     possible_steps.truncate(previous - 1);
        //     steps.truncate(previous - 1);
        // }

        // if p_s.len() > 0 {
        //     println!("");
        //     println!("Could've already jumped here: {:?}", p_s);
        //     println!("");
        // }

        possible_steps.push(
            available_steps
                .into_iter()
                .flatten()
                .map(|(p, _)| p)
                .collect::<Vec<Position>>(),
        );

        // println!("{:?} Steping to {:?}", direction_order, next_step.0);
        // println!("");
        return next_step.0;
    }

    // If there is no step available, list this current step as blocked step
    blocked_steps.push(*current_step);
    steps.pop();

    let fallback = *steps.last().unwrap();

    fallback
}

fn get_direction_order() -> String {
    let mut directions = vec!["U", "R", "D", "L"];
    fastrand::shuffle(&mut directions);

    let mut result = String::new();
    directions.iter().for_each(|c| result.push_str(c));

    result
}

fn build_board(data: &str) -> (Position, Vec<Vec<Position>>) {
    let mut start: Position = Position {
        x: 0,
        y: 0,
        height: 0,
    };

    let board: Vec<Vec<Position>> = data
        .lines()
        .enumerate()
        .map(|(vertical_index, item)| {
            item.chars()
                .enumerate()
                .map(|(horizontal_index, c)| {
                    let height = match ALPHABET.find(c) {
                        Some(n) => n,
                        _ => {
                            if c == 'S' {
                                start = Position {
                                    x: horizontal_index,
                                    y: vertical_index,
                                    height: 0,
                                };

                                return start;
                            }
                            if c == 'E' {
                                return Position {
                                    x: horizontal_index,
                                    y: vertical_index,
                                    height: 26,
                                };
                            }
                            0
                        }
                    };

                    Position {
                        x: horizontal_index,
                        y: vertical_index,
                        height,
                    }
                })
                .collect::<Vec<Position>>()
        })
        .collect();

    (start, board)
}
