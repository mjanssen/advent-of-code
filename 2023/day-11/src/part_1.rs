#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    char: char,
}

fn parse_data(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            if line.contains("#") {
                return vec![line];
            }

            return vec![line, line];
        })
        .fold(vec![vec![]], |mut matrix, line| {
            for (char_index, char) in line.chars().enumerate() {
                match matrix.get_mut(char_index) {
                    Some(matrix_line) => matrix_line.push(char),
                    _ => matrix.push(vec![char]),
                }
            }

            matrix
        })
        .into_iter()
        .flat_map(|line| {
            if line.contains(&'#') {
                return vec![line];
            }

            return vec![line.clone(), line.clone()];
        })
        .enumerate()
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
        .fold(
            (vec![] as Vec<Node>, 0),
            |(mut galaxies, mut answer), node| {
                for galaxy in &galaxies {
                    let diff_x = galaxy.x.abs_diff(node.x);
                    let diff_y = galaxy.y.abs_diff(node.y);
                    answer += diff_x + diff_y;
                }

                galaxies.push(node);

                (galaxies, answer)
            },
        )
        .1
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(parse_data(input).to_string())
}
