use itertools::Itertools;

fn determine_safety(left: &u32, right: &u32, safe: &mut bool) {
    let n = left.abs_diff(*right);
    if n == 0 || n > 3 {
        *safe = false;
    }
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = _input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse::<u32>().ok())
                .collect()
        })
        .filter(|chars: &Vec<u32>| {
            let mut safe: bool = true;

            match chars.get(0).unwrap().lt(chars.get(1).unwrap()) {
                true => {
                    // ascending
                    for (left, right) in chars.iter().tuple_windows() {
                        if right > left && safe == true {
                            determine_safety(left, right, &mut safe);
                        } else {
                            safe = false;
                        }
                    }
                }
                false => {
                    for (left, right) in chars.iter().tuple_windows() {
                        if right < left && safe == true {
                            determine_safety(left, right, &mut safe);
                        } else {
                            safe = false;
                        }
                    }
                }
            };

            if safe {
                println!("{:?}", chars)
            }

            safe
        })
        .count();

    Ok(res.to_string())
}
