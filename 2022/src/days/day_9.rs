use std::collections::HashSet;

use itertools::Itertools;

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

const PART_AMOUNT: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn positions_touch(current: &Position, target: &Position) -> bool {
    if (target.y - current.y) > 1 {
        return false;
    }

    if (target.y - current.y) < -1 {
        return false;
    }

    if (target.x - current.x) > 1 {
        return false;
    }

    if (target.x - current.x) < -1 {
        return false;
    }

    true
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_9.txt")?;

    // part 1 example - 0, 5
    // part 2 example - 11,15
    let start_pos = (11, 15);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(start_pos);

    let start_position = Position {
        x: start_pos.0,
        y: start_pos.1,
    };

    let mut parts = vec![start_position; PART_AMOUNT];

    data.lines().for_each(|line| {
        if let [direction, s] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            let steps = s.parse::<i32>().unwrap();

            (0..steps).for_each(|_| {
                // First let the front position make a move
                let current_position: &mut Position = &mut parts[0];

                match direction {
                    "L" => current_position.x -= 1,
                    "R" => current_position.x += 1,
                    "U" => current_position.y -= 1,
                    "D" => current_position.y += 1,
                    _ => (),
                }

                for (head, tail) in (0..parts.len()).tuple_windows() {
                    let touching = positions_touch(&parts[tail], &parts[head]);

                    if touching == false {
                        parts[tail].x += (parts[head].x - parts[tail].x).signum();
                        parts[tail].y += (parts[head].y - parts[tail].y).signum();

                        if tail == PART_AMOUNT - 1 {
                            let pos = parts[PART_AMOUNT - 1];
                            visited.insert((pos.x, pos.y));
                        }
                    }
                }
            });
        };
    });

    println!("{:?}", visited.len());

    // Part to visualize the drawing
    // (0..21).into_iter().for_each(|y| {
    //     (0..26).into_iter().for_each(|x| {
    //         if visited.contains(&(x, y)) {
    //             print!(" x ");
    //         } else {
    //             print!(" . ");
    //         }
    //     });
    //     println!("");
    // });

    Ok(())
}
