use std::collections::HashMap;

use itertools::Itertools;

const SEARCH: [&str; 2] = ["MS", "SM"];

fn dir_iter() -> Vec<i16> {
    (-1..2).collect_vec()
}

trait CollectMatches {
    fn collect_matches(self) -> String;
}

impl<'a, I> CollectMatches for I
where
    I: Iterator<Item = &'a char>,
{
    fn collect_matches(self) -> String {
        self.filter(|&&c| matches!(c, 'M' | 'S')).copied().collect()
    }
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
        .filter(|(_, char)| char == &&"A".chars().next().unwrap())
        .map(|((x, y), _)| {
            let mut count = 0;

            let left_up = dir_iter()
                .into_iter()
                .filter_map(|i| map.get(&(x - i, y - i)))
                .collect_matches();

            let right_up = dir_iter()
                .into_iter()
                .filter_map(|i| map.get(&(x + i, y - i)))
                .collect_matches();

            if SEARCH.contains(&right_up.as_str()) && SEARCH.contains(&left_up.as_str()) {
                count += 1;
            }

            count
        })
        .sum::<u16>();

    Ok(res.to_string())
}
