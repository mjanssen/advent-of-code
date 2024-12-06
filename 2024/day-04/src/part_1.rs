use std::{collections::HashMap, ops::RangeInclusive};

const SEARCH: [&str; 1] = ["XMAS"];

fn dir_iter() -> RangeInclusive<i16> {
    (0..=3).into_iter()
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let map = _input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i16, y as i16), char))
        })
        .collect::<HashMap<(i16, i16), _>>();

    let res = map
        .iter()
        .map(|((x, y), _)| {
            let mut count = 0;

            // Left
            let left = dir_iter()
                .filter_map(|i| map.get(&(x + i, *y)))
                .collect::<String>();

            if SEARCH.contains(&left.as_str()) {
                count += 1;
            }

            let right = dir_iter()
                .filter_map(|i| map.get(&(x - i, *y)))
                .collect::<String>();

            if SEARCH.contains(&right.as_str()) {
                count += 1;
            }

            let up = dir_iter()
                .filter_map(|i| map.get(&(*x, y - i)))
                .collect::<String>();

            if SEARCH.contains(&up.as_str()) {
                count += 1;
            }

            let down = dir_iter()
                .filter_map(|i| map.get(&(*x, y + i)))
                .collect::<String>();

            if SEARCH.contains(&down.as_str()) {
                count += 1;
            }

            let left_up = dir_iter()
                .filter_map(|i| map.get(&(x - i, y - i)))
                .collect::<String>();

            if SEARCH.contains(&left_up.as_str()) {
                count += 1;
            }

            let right_up = dir_iter()
                .filter_map(|i| map.get(&(x + i, y - i)))
                .collect::<String>();

            if SEARCH.contains(&right_up.as_str()) {
                count += 1;
            }

            let left_down = dir_iter()
                .filter_map(|i| map.get(&(x - i, y + i)))
                .collect::<String>();

            if SEARCH.contains(&left_down.as_str()) {
                count += 1;
            }

            let right_down = dir_iter()
                .filter_map(|i| map.get(&(x + i, y + i)))
                .collect::<String>();

            if SEARCH.contains(&right_down.as_str()) {
                count += 1;
            }

            count
        })
        .sum::<u16>();

    Ok(res.to_string())
}
