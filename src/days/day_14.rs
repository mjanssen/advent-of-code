use std::{cmp::Ordering, collections::HashSet};

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Rock {
    x: usize,
    y: usize,
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_14.txt")?;

    let rocks = parse_rocks(&data);

    Ok(())
}

fn parse_rocks(data: &str) -> HashSet<Rock> {
    let mut set = HashSet::new();

    let mut x_ranges: HashSet<usize> = HashSet::new();
    let mut y_ranges: HashSet<usize> = HashSet::new();

    for line in data.lines() {
        let mut current_range: Vec<(usize, usize)> = Vec::new();

        for range in line
            .split(" -> ")
            .map(|range_item| range_item.split(",").collect::<Vec<&str>>())
        {
            // X
            let range_x_start: usize = match range[0].parse() {
                Ok(n) => n,
                _ => 0,
            };

            // Y
            let range_y_end: usize = match range[1].parse() {
                Ok(n) => n,
                _ => 0,
            };

            if let Some(last) = current_range.last() {
                // If X equals -> range from Y
                if last.0 == range_x_start {
                    for i in last.1..=range_y_end {
                        set.insert(Rock {
                            x: last.0 as usize,
                            y: i as usize,
                        });
                        y_ranges.insert(i);
                    }
                } else if last.1 == range_y_end {
                    // If Y equals -> range from X
                    for i in last.0..=range_x_start {
                        set.insert(Rock {
                            x: i as usize,
                            y: last.1 as usize,
                        });
                        x_ranges.insert(i);
                    }
                }
            }

            current_range.push((range_x_start, range_y_end));
        }
    }

    println!("{:?}", set);
    println!("{:?}", x_ranges.len());
    println!("{:?}", y_ranges.len());

    set
}
