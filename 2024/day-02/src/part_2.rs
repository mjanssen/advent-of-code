use itertools::Itertools;

enum Order {
    Asc,
    Desc,
}

fn failed(diff: &i32) -> bool {
    match diff.abs() {
        n if n == 0 || n > 3 => return true,
        _ => return false,
    }
}

fn determine_safety(chars: &Vec<i32>) -> bool {
    let mut order: Option<Order> = None;
    let mut valid = true;

    for (left, right) in chars.into_iter().tuple_windows() {
        let diff = left - right;
        match diff.signum() {
            0 => return false,
            1 => match &order {
                None => {
                    order = Some(Order::Desc);
                    if failed(&diff) {
                        valid = false;
                    }
                }
                Some(Order::Asc) => return false,
                Some(Order::Desc) => {
                    if failed(&diff) {
                        valid = false;
                    }
                }
            },
            -1 => match &order {
                None => {
                    order = Some(Order::Asc);
                    if failed(&diff) {
                        valid = false;
                    }
                }
                Some(Order::Desc) => return false,
                Some(Order::Asc) => {
                    if failed(&diff) {
                        valid = false;
                    }
                }
            },
            _ => panic!("¯\\_(ツ)_/¯"),
        }
    }

    valid
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = _input
        .lines()
        .filter(|line| {
            let chars: Vec<i32> = line
                .split_whitespace()
                .filter_map(|v| v.parse::<i32>().ok())
                .collect();

            if determine_safety(&chars) == false {
                for i in 0..chars.len() {
                    let mut modified = chars.clone();
                    modified.remove(i);
                    if determine_safety(&modified) {
                        return true;
                    } else {
                        continue;
                    }
                }

                return false;
            }

            return true;
        })
        .count();

    Ok(res.to_string())
}
