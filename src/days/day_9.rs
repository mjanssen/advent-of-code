use std::collections::HashSet;

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

const PART_AMOUNT: usize = 9;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    previous_x: i32,
    previous_y: i32,
}

fn positions_touch(current: &(i32, i32), target: &(i32, i32)) -> bool {
    if (target.1 - current.1) > 1 {
        return false;
    }

    if (target.1 - current.1) < -1 {
        return false;
    }

    if (target.0 - current.0) > 1 {
        return false;
    }

    if (target.0 - current.0) < -1 {
        return false;
    }

    true
}

fn get_next_position(current: &(i32, i32), target: &(i32, i32)) -> (i32, i32) {
    let target_tuple = &(target.0, target.1);

    // If they already touch, just return current position
    if positions_touch(&(current.0, current.1), target_tuple) {
        return (current.0, current.1);
    }

    let x_plus_one = (current.0 + 1, current.1);
    if positions_touch(&x_plus_one, target_tuple) {
        return x_plus_one;
    }

    let x_min_one = (current.0 - 1, current.1);
    if positions_touch(&x_min_one, target_tuple) {
        return x_min_one;
    }

    let y_plus_one = (current.0, current.1 + 1);
    if positions_touch(&y_plus_one, target_tuple) {
        return y_plus_one;
    }

    let y_min_one = (current.0, current.1 - 1);
    if positions_touch(&y_min_one, target_tuple) {
        return y_min_one;
    }

    let lu_diag = (current.0 - 1, current.1 - 1);
    if positions_touch(&lu_diag, target_tuple) {
        println!("lu diag");
        return lu_diag;
    }

    let ld_diag = (current.0 - 1, current.1 + 1);
    if positions_touch(&ld_diag, target_tuple) {
        println!("ld diag");
        return ld_diag;
    }

    let ru_diag = (current.0 + 1, current.1 - 1);
    if positions_touch(&ru_diag, target_tuple) {
        println!("ru diag");
        return ru_diag;
    }

    let rd_diag = (current.0 + 1, current.1 + 1);
    if positions_touch(&rd_diag, target_tuple) {
        println!("rd diag");
        return rd_diag;
    }

    target_tuple.to_owned()
}

fn snapshot(start: Position, positions: &[Position; PART_AMOUNT]) -> () {
    println!("{:?}", positions);
    (0..21).into_iter().for_each(|y| {
        (0..26).into_iter().for_each(|x| {
            let pos: Vec<(i32, i32, usize)> = positions
                .iter()
                .enumerate()
                .filter(|(i, pos)| pos.x == x && pos.y == y)
                .map(|(i, pos)| (pos.x, pos.y, i))
                .collect();

            if start.x == x && start.y == y {
                print!(" s ");
            } else if let Some(pos) = pos.first() {
                print!(" {} ", pos.2);
            } else {
                print!(" . ");
            }
        });
        println!("");
    });
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_9.txt")?;

    // part 1 example - 0, 5
    // part 2 example - 11,15
    let start_pos = (11, 15);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_head: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(start_pos);
    visited_head.insert(start_pos);

    let start_position = Position {
        x: start_pos.0,
        y: start_pos.1,
        previous_x: 0,
        previous_y: 0,
    };

    const PUZZLE_PART: u8 = 2;

    let mut positions: [Position; PART_AMOUNT] = [
        start_position.clone(),
        start_position.clone(),
        start_position.clone(),
        start_position.clone(),
        start_position.clone(),
        start_position.clone(),
        start_position.clone(),
        start_position.clone(),
        start_position.clone(),
    ];

    data.lines().for_each(|line| {
        if let [direction, s] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            let steps = s.parse::<i32>().unwrap();

            (0..steps).for_each(|_| {
                // First let the front position make a move
                let current_position: &mut Position = &mut positions[0];

                current_position.previous_x = current_position.x;
                current_position.previous_y = current_position.y;

                if direction == "R" {
                    current_position.x += 1;
                }

                if direction == "L" {
                    current_position.x -= 1;
                }

                if direction == "U" {
                    current_position.y -= 1;
                }

                if direction == "D" {
                    current_position.y += 1;
                }

                visited_head.insert((current_position.x, current_position.y));

                println!(
                    "{} {} {:?} {:?}",
                    direction,
                    s,
                    (current_position.previous_x, current_position.previous_y),
                    (current_position.x, current_position.y)
                );

                for part in 1..PART_AMOUNT {
                    let previous_part = positions[part - 1];
                    let current_position: &mut Position = &mut positions[part];

                    if PUZZLE_PART == 1 {
                        let touching = positions_touch(
                            &(current_position.x, current_position.y),
                            &(previous_part.x, previous_part.y),
                        );
                        if touching == false {
                            current_position.previous_x = current_position.x;
                            current_position.previous_y = current_position.y;
                            current_position.x = previous_part.previous_x;
                            current_position.y = previous_part.previous_y;

                            if part == 1 {
                                visited.insert((current_position.x, current_position.y));
                            }
                        }
                    }

                    if PUZZLE_PART == 2 {
                        let touching = positions_touch(
                            &(current_position.x, current_position.y),
                            &(previous_part.x, previous_part.y),
                        );

                        let next_pos = get_next_position(
                            &(current_position.x, current_position.y),
                            &(previous_part.x, previous_part.y),
                        );
                        // if next_pos.0 != current_position.x && next_pos.1 != current_position.y {

                        current_position.x = next_pos.0;
                        current_position.y = next_pos.1;

                        if part == 2 {
                            visited.insert((current_position.x, current_position.y));
                        }
                        // }

                        println!(
                            "{} {} {:?} {:?}",
                            touching,
                            part,
                            next_pos,
                            (current_position.x, current_position.y)
                        );
                    }
                }

                println!("");
            });
        };
    });

    snapshot(start_position, &positions);

    println!("{:?}", visited.len());

    // (0..21).into_iter().for_each(|y| {
    //     (0..26).into_iter().for_each(|x| {
    //         if visited.contains(&(x, y)) {
    //             print!(" x ");
    //         } else if visited_head.contains(&(x, y)) {
    //             print!(" O ");
    //         } else {
    //             print!(" . ");
    //         }
    //     });
    //     println!("");
    // });

    Ok(())
}
