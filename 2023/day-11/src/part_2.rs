use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    char: char,
}

fn parse_data(input: &str) -> usize {
    let mut empty_lines: (HashSet<usize>, HashSet<usize>) = (HashSet::new(), HashSet::new());

    let grid = input
        .lines()
        .enumerate()
        .inspect(|(y_index, line)| {
            if line.contains(&"#") == false {
                empty_lines.0.insert(*y_index);
            }
        })
        .fold(vec![vec![]], |mut matrix, (_, line)| {
            for (char_index, char) in line.chars().enumerate() {
                match matrix.get_mut(char_index) {
                    Some(matrix_line) => matrix_line.push(char),
                    _ => matrix.push(vec![char]),
                }
            }

            matrix
        })
        .iter()
        .enumerate()
        .inspect(|(x_index, line)| {
            if line.contains(&'#') == false {
                empty_lines.1.insert(*x_index);
            }
        })
        .map(|(line_index, chars)| {
            chars
                .iter()
                .enumerate()
                .map(|(char_index, char)| Node {
                    x: char_index,
                    y: line_index,
                    char: *char,
                })
                .collect::<Vec<Node>>()
        })
        .flat_map(|nodes| {
            nodes
                .into_iter()
                .filter(|n| n.char.ne(&'.'))
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Node>>();

    let mut answer = 0;

    for (i, base_node) in grid.iter().enumerate() {
        for galaxy_index in i + 1..grid.len() {
            let galaxy = &grid[galaxy_index];

            let distance_x = base_node.x.abs_diff(galaxy.x);
            let distance_y = base_node.y.abs_diff(galaxy.y);

            let range_x_start = if base_node.x < galaxy.x {
                base_node.x
            } else {
                galaxy.x
            };

            let x = (range_x_start + 1..=range_x_start + distance_x)
                .map(|f| {
                    if empty_lines.0.contains(&f) {
                        return 1_000_000;
                    }

                    return 1;
                })
                .sum::<usize>();

            let range_y_start = if base_node.y < galaxy.y {
                base_node.y
            } else {
                galaxy.y
            };

            let y = (range_y_start + 1..=range_y_start + distance_y)
                .map(|f| {
                    if empty_lines.1.contains(&f) {
                        return 1_000_000;
                    }

                    return 1;
                })
                .sum::<usize>();

            answer += x + y;
        }
    }

    answer
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(parse_data(input).to_string())
}
