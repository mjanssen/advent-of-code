fn check_around(grid: &Vec<Vec<i32>>, line_index: usize, char_index: usize) -> bool {
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

    for (_, line_index) in (line_start..=line_end).enumerate() {
        for (_, char_index) in (char_start..=char_end).enumerate() {
            if grid[line_index][char_index].eq(&1) {
                return true;
            }
        }
    }

    println!("{line_start} {line_end}");

    false
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let size = &_input.lines().collect::<Vec<&str>>();
    let mut grid = vec![vec![0; size[0].len()]; size.len()];

    // Build grid to check references
    for (i, line) in _input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            match char {
                x if x.eq(&'*') => grid[i][char_index] = 1,
                _ => (),
            }
        }
    }

    let answer = _input
        .lines()
        .enumerate()
        .fold(0, |mut ans, (line_index, line)| {
            let mut number = "".to_string();
            let mut adjacent_to_symbol = false;

            for (char_index, char) in line.chars().enumerate() {
                match char {
                    x if x.is_digit(10) => {
                        number += &x.to_string();
                        if adjacent_to_symbol == false {
                            adjacent_to_symbol = check_around(&grid, line_index, char_index);
                        }
                    }
                    _ => {
                        if number.len() > 0 && adjacent_to_symbol {
                            ans += number.parse::<i32>().unwrap_or(0);
                        }

                        number = "".to_string();
                        adjacent_to_symbol = false;
                    }
                }
            }

            // If the number was listed at the end of the line, make sure we add it
            if number.len() > 0 && adjacent_to_symbol {
                ans += number.parse::<i32>().unwrap_or(0);
            }

            ans
        });

    Ok(answer.to_string())
}
