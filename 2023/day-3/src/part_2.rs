use std::collections::HashSet;
use std::ops::RangeInclusive;

fn get_range(
    grid: &Vec<Vec<i32>>,
    line_index: usize,
    char_index: usize,
) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let line_start = if line_index > 0 { line_index - 1 } else { 0 };
    let line_end = if line_index < grid.len() - 1 {
        line_index + 1
    } else {
        line_index
    };

    let char_start = if char_index > 0 { char_index - 1 } else { 0 };
    let char_end = if char_index < grid[0].len() - 1 {
        char_index + 1
    } else {
        char_index
    };

    ((line_start..=line_end), (char_start..=char_end))
}

fn get_around(grid: &Vec<Vec<i32>>, line_index: usize, char_index: usize) -> HashSet<String> {
    let mut result = HashSet::new();

    let (line_range, char_range) = get_range(grid, line_index, char_index);

    for (_, line_index) in line_range.clone().enumerate() {
        for (_, char_index) in char_range.clone().enumerate() {
            let item = grid[line_index][char_index];
            if item.ne(&0) && item.ne(&-1) {
                result.insert(
                    grid[line_index][char_index].to_string() + ":" + &line_index.to_string(),
                );
            }
        }
    }

    result
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let size = &_input.lines().collect::<Vec<&str>>();
    let mut grid = vec![vec![0; size[0].len()]; size.len()];

    // Build grid to check references
    for (i, line) in _input.lines().enumerate() {
        let mut number = "".to_string();
        let mut number_start = 0;

        for (char_index, char) in line.chars().enumerate() {
            match char {
                x if x.eq(&'*') => {
                    if number.len() > 0 {
                        for (_, ci) in (number_start..char_index).enumerate() {
                            grid[i][ci] = number.parse::<i32>().unwrap_or(0);
                        }
                    }

                    // Mark apostrophe as -1 in grid, since there is no -1 value
                    grid[i][char_index] = -1;

                    number = "".to_string();
                    number_start = 0;
                }
                x if x.is_digit(10) => {
                    if number.len() == 0 {
                        number_start = char_index;
                    }

                    number += &x.to_string();
                }
                _ => {
                    if number.len() > 0 {
                        for (_, ci) in (number_start..char_index).enumerate() {
                            grid[i][ci] = number.parse::<i32>().unwrap_or(0);
                        }
                    }

                    number = "".to_string();
                    number_start = 0;
                }
            }
        }

        if number.len() > 0 {
            for (_, ci) in (number_start..=line.len() - 1).enumerate() {
                grid[i][ci] = number.parse::<i32>().unwrap_or(0);
            }
        }
    }

    let mut answer = 0;

    for (line_index, line) in grid.iter().enumerate() {
        for (num_index, num) in line.iter().enumerate() {
            if num.eq(&-1) {
                let around = get_around(&grid, line_index, num_index);
                if around.len() == 2 {
                    let add = around.iter().fold(1, |mut a, num| {
                        if let Ok(n) = num
                            .split(":")
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap_or(&"0")
                            .parse::<i32>()
                        {
                            a = a * n;
                        }

                        a
                    });

                    answer += add;
                }
            }
        }
    }

    Ok(answer.to_string())
}
