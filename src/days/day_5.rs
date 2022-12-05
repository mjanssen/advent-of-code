use std::collections::VecDeque;

use regex::Regex;

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

fn run(input: &Vec<&str>, part_2: bool) {
    let boxes: &str = match input.get(0) {
        Some(b) => b,
        _ => &"",
    };

    let replace_regex = Regex::new("\\W").unwrap();

    let mut vertical_lines: Vec<VecDeque<&str>> = vec![];

    // Parse box lines to usable vectors
    let mut parsed_box_lines: Vec<Vec<String>> = boxes
        .lines()
        .map(|line| {
            let parsed_line: String = line
                .replace("    ", ".")
                .replace(" ", "")
                .replace(".", "[]");

            let line_segments: Vec<String> = parsed_line
                .split("][")
                .map(|item| replace_regex.replace(item, "").to_string())
                .collect();

            line_segments
        })
        .collect();

    // Strip out the row numbers
    parsed_box_lines = parsed_box_lines[0..parsed_box_lines.len() - 1].to_vec();

    parsed_box_lines.iter().for_each(|items| {
        items
            .iter()
            .enumerate()
            .for_each(|(horizontal_index, item)| {
                let mut existing: VecDeque<&str> = match vertical_lines.get(horizontal_index) {
                    Some(ex) => ex.to_owned(),
                    _ => {
                        let v = VecDeque::new();
                        vertical_lines.push(v);
                        vertical_lines.get(horizontal_index).unwrap().to_owned()
                    }
                };

                if item.ne(&"".to_string()) {
                    existing.push_front(item);
                }

                vertical_lines[horizontal_index] = existing
            });
    });

    let input: &str = match input.get(1) {
        Some(b) => b,
        _ => &"",
    };

    let move_regex = Regex::new("[0-9]+").unwrap();

    input.lines().for_each(|mv| {
        let move_numbers = move_regex
            .find_iter(mv)
            .map(|x| x.as_str())
            .collect::<Vec<&str>>();

        let box_numb: usize = match move_numbers.get(0) {
            Some(n) => n.parse::<usize>().unwrap_or(0),
            _ => 0,
        };

        let from: usize = match move_numbers.get(1) {
            Some(n) => n.parse::<usize>().unwrap_or(1),
            _ => 1,
        } - 1;

        let to: usize = match move_numbers.get(2) {
            Some(n) => n.parse::<usize>().unwrap_or(1),
            _ => 1,
        } - 1;

        if box_numb != 0 {
            let from_box = &vertical_lines
                .get(from)
                .expect("Expected from box to be available");

            let target_box = &mut vertical_lines
                .get(to)
                .expect("Expected to box to be available")
                .to_owned();

            let mut append_to: VecDeque<&str> = VecDeque::new();

            from_box
                .iter()
                .rev()
                .take(box_numb)
                .for_each(|item: &&str| {
                    append_to.push_back(item);
                });

            if part_2 {
                append_to.iter().rev().for_each(|s| {
                    target_box.push_back(s);
                });
            } else {
                append_to.iter().for_each(|s| {
                    target_box.push_back(s);
                });
            }

            let new_values: VecDeque<&str> = from_box
                .to_owned()
                .range(0..(from_box.len() - box_numb))
                .map(|s| s.to_owned())
                .collect();

            vertical_lines[from] = new_values.to_owned();
            vertical_lines[to] = target_box.to_owned();
        }
    });

    let res: Vec<String> = vertical_lines
        .iter()
        .map(|line| line.get(line.len() - 1).unwrap().to_string())
        .collect();

    if part_2 {
        println!("pt 2 - {}", res.join(""));
    } else {
        println!("pt 1 - {}", res.join(""));
    }
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_5.txt")?;
    let input: Vec<&str> = data.split("\n\n").collect();

    run(&input, false);
    run(&input, true);

    Ok(())
}
