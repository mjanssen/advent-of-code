use std::cmp::Ordering;

use serde_json::{json, Value};

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

#[derive(Debug, PartialEq)]
struct Pair {
    number: usize,
    left: Value,
    right: Value,
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_13.txt")?;
    let pairs: Vec<Pair> = parse(&data);

    // Part 1
    let mut score = 0;
    for pair in pairs.iter() {
        let order = compare(&pair.left, &pair.right);

        if order.is_lt() {
            score += pair.number;
        }
    }

    println!("part 1 - {}", score);

    // Part 2 - Create new unordered vec with all items
    let mut unordered = pairs.into_iter().fold(Vec::new(), |mut acc, item| {
        acc.push(item.left);
        acc.push(item.right);

        acc
    });

    let add_arr_two: Value = serde_json::from_str("[[2]]").unwrap();
    let add_arr_six: Value = serde_json::from_str("[[6]]").unwrap();

    unordered.push(add_arr_two.clone());
    unordered.push(add_arr_six.clone());

    let mut ordered: Vec<_> = Vec::new();
    ordered.push(&unordered[0]);

    for item in unordered.iter() {
        for (index, current) in ordered.clone().iter().enumerate() {
            let exists: Vec<&&Value> = ordered.iter().filter(|f| f.to_owned().eq(&item)).collect();
            // Skip if it already exists in the ordered list
            if exists.len() > 0 {
                break;
            }

            let order = compare(item, current);

            // If item < current item, insert before
            if order.is_lt() {
                ordered.insert(index, item);
                break;
            }

            // If greater than last item, push it to the end (no break needed)
            if order.is_eq() && index == ordered.len() - 1 {
                ordered.push(item);
                break;
            }

            // If greater than last item, push it to the end (no break needed)
            if order.is_gt() && index == ordered.len() - 1 {
                ordered.push(item);
            }
        }
    }

    let part_2: usize = ordered.into_iter().enumerate().fold(1, |acc, (i, item)| {
        if item.clone().eq(&add_arr_two) || item.clone().eq(&add_arr_six) {
            return acc * (i + 1);
        }

        acc
    });

    println!("part 2 - {}", part_2);

    Ok(())
}

fn compare(first: &Value, second: &Value) -> Ordering {
    match (first, second) {
        (Value::Array(left), Value::Array(right)) => {
            let mut index = 0;

            while index < left.len() && index < right.len() {
                // If there are still items available
                match (left[index].clone(), right[index].clone()) {
                    // If they're both numbers, match them if they're unequal and return ordering
                    (Value::Number(l), Value::Number(r)) => {
                        if l.as_u64() != r.as_u64() {
                            return l.as_u64().unwrap().cmp(&r.as_u64().unwrap());
                        }
                    }
                    // Recursive strategy below
                    // If left side is array and right side is number
                    (Value::Array(l), Value::Number(r)) => {
                        let check = compare(&Value::from(l), &json!([r.as_u64()]));
                        if check.is_eq() == false {
                            return check;
                        }
                    }
                    // If right side is array and left side is number
                    (Value::Number(l), Value::Array(r)) => {
                        let check = compare(&json!([l.as_u64()]), &Value::from(r));
                        if check.is_eq() == false {
                            return check;
                        }
                    }
                    // If left and right side are array
                    (Value::Array(l), Value::Array(r)) => {
                        let check = compare(&Value::from(l), &Value::from(r));
                        if check.is_eq() == false {
                            return check;
                        }
                    }
                    _ => (),
                };

                // Go to next item in list
                index += 1;
            }

            left.len().cmp(&right.len())
        }
        _ => panic!("???"),
    }
}

fn parse(data: &str) -> Vec<Pair> {
    let mut items: Vec<Value> = vec![];
    let mut result: Vec<Pair> = vec![];

    data.split("\n\n").for_each(|line: &str| {
        line.split("\n")
            .filter(|i| i.is_empty() == false)
            .for_each(|item| {
                if let Ok(val) = serde_json::from_str(item) {
                    items.push(val);
                }
            });
    });

    let mut pair_no = 1;
    for i in (0..items.len()).step_by(2) {
        let first = &items[i];
        let second = &items[i + 1];

        result.push(Pair {
            number: pair_no,
            left: first.clone(),
            right: second.clone(),
        });

        pair_no += 1;
    }

    result
}
