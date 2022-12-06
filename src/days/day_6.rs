use std::{cmp::Ordering, collections::HashSet};

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

#[derive(PartialEq, Debug)]
struct Unique {
    char_set: HashSet<char>,
    index: usize,
    marker: usize,
}

fn check(data: &str, length: usize) -> () {
    let mut unique: Option<Unique> = None;

    let chars = data.chars().collect::<Vec<char>>();

    chars.iter().enumerate().for_each(|(i, _)| {
        if unique == None {
            let end_index = match (i + length).cmp(&chars.len()) {
                Ordering::Less => i + length,
                Ordering::Greater => i + (&chars.len() - i),
                Ordering::Equal => i + length,
            };

            let char_set: &mut Vec<char> = &mut chars[i..end_index].to_vec();

            if char_set.len() == length {
                let set: HashSet<char> = char_set.drain(..).collect();

                if set.len() == length {
                    unique = Some(Unique {
                        char_set: set,
                        index: i,
                        marker: end_index,
                    });
                }
            }
        }
    });

    println!("{:?}", unique);
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_6.txt")?;

    check(&data, 4);
    check(&data, 14);

    Ok(())
}
