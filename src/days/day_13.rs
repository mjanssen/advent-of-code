use serde_json::{json, Value};

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

#[derive(Debug, Clone)]
struct Pair {
    number: usize,
    left: Value,
    right: Value,
}

// #[derive(Debug)]
// enum Item {
//     Number(u32),
//     List(Vec<Item>),
// }

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_13.txt")?;
    let pairs: Vec<Pair> = parse(&data);

    let mut ordered: Vec<Pair> = vec![];
    let mut unordered: Vec<Pair> = vec![];

    for pair in pairs.iter() {
        // If left is empty, it's ordered
        // if pair.left.is_null() {
        //     ordered.push(pair.clone());
        //     continue;
        // }

        // // If right is empty, not ordered
        // if pair.right.is_null() {
        //     unordered.push(pair.clone());
        //     continue;
        // }

        let left = pair.left.as_array().unwrap();
        let right = pair.right.as_array().unwrap();

        if compare_array(left, right, false) {
            ordered.push(pair.to_owned());
        } else {
            unordered.push(pair.to_owned());
        }
    }

    let score = ordered.iter().fold(0, |acc, p| acc + p.number);

    println!("part 1 - {}", score);

    Ok(())
}

fn compare_array(first: &Vec<Value>, second: &Vec<Value>, mixed: bool) -> bool {
    let mut ordered: bool = true;

    println!("f {:?}", first);
    println!("s {:?}", second);

    // Left side ran out of items, so inputs are in the right order
    if first.is_empty() && second.is_empty() == false {
        return true;
    }

    // Right side ran out of items, so inputs are not in the right order
    if first.is_empty() == false && second.is_empty() {
        return false;
    }

    // Mixed only checks one item
    if mixed {
        if let Some(f) = first.get(0) {
            if let Some(s) = second.get(0) {
                if let Some(f_num) = f.as_u64() {
                    if let Some(s_num) = s.as_u64() {
                        if f_num > s_num {
                            return false;
                        } else {
                            return true;
                        }
                    }
                }
            }
        }
    }

    let first_has_all_numbers: bool = first.iter().filter(|p| p.is_number() == false).count() == 0;

    let second_has_all_numbers: bool =
        second.iter().filter(|p| p.is_number() == false).count() == 0;

    // Check if first array has been exhausted
    // if first_has_all_numbers && second_has_all_numbers && second.len() < first.len() {
    //     println!("ff");
    //     return false;
    // }

    println!("");

    for (i, left) in first.iter().enumerate() {
        if let Some(right) = second.get(i) {
            // If they're both arrays, recursively return this method
            if left.is_array() && right.is_array() {
                return compare_array(left.as_array().unwrap(), right.as_array().unwrap(), mixed);
            }

            if left.is_number() && right.is_number() {
                if left.as_u64() > right.as_u64() {
                    ordered = false;
                    break;
                }
            }

            // If left is number and right is array
            if left.is_number() && right.is_array() {
                println!("l: {:?} \nr: {:?} \n {:?}", left, right, ordered);
                println!("");
                println!("{:?}", json!([left.as_u64()]).as_array().unwrap());
                return compare_array(
                    json!([left.as_u64()]).as_array().unwrap(),
                    right.as_array().unwrap(),
                    true,
                );
            }

            // If right is number and left is array
            if left.is_array() && right.is_number() {
                return compare_array(
                    left.as_array().unwrap(),
                    json!([right.as_u64()]).as_array().unwrap(),
                    true,
                );
            }
        }
    }

    // first.iter().enumerate().for_each(|(i, left)| {
    //     if let Some(right) = second.get(i) {
    //         if right.is_number() && left.is_number() {
    //             if left.as_u64() > right.as_u64() {
    //                 ordered = false;
    //                 return;
    //             }
    //         }

    //         if left.is_array() && right.is_array() {
    //             ordered = compare_array(left.as_array().unwrap(), right.as_array().unwrap(), mixed);
    //             break ordered;
    //         }

    //         // If left is number and right is array
    //         if left.is_number() && right.is_array() {
    //             ordered = compare_array(
    //                 json!([left.as_u64()]).as_array().unwrap(),
    //                 right.as_array().unwrap(),
    //                 true,
    //             );
    //             return;
    //         }

    //         // If right is number and left is array
    //         if left.is_array() && right.is_number() {
    //             ordered = compare_array(
    //                 left.as_array().unwrap(),
    //                 json!([right.as_u64()]).as_array().unwrap(),
    //                 true,
    //             );
    //             return;
    //         }
    //     }
    // });

    ordered
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
