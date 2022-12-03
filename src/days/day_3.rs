use crate::lib::load_file::load_data_file;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug)]
struct Line<'a> {
    pt_1: &'a str,
    pt_2: &'a str,
    dupes: Vec<String>,
    score: Option<usize>,
}

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_file("day_3.txt")?;
    let score: usize = data
        .lines()
        .map(|line| {
            let (pt_1, pt_2) = line.split_at(line.len() / 2);
            let mut line = Line {
                pt_1,
                pt_2,
                dupes: Vec::new(),
                score: None,
            };

            let char_set: &Vec<char> = &line.pt_2.chars().collect();

            let _ = &char_set
                .iter()
                .filter(|x: &&char| line.pt_1.contains(&x.to_string()))
                .map(|x: &char| line.dupes.push(x.to_string()))
                .collect::<Vec<()>>();

            line.dupes.iter().for_each(|x| {
                if line.score == None {
                    if let Some(pos) = ALPHABET.chars().position(|c| c.to_string().eq(x)) {
                        // Plus one since index starts at 0
                        line.score = Some(pos + 1);
                    }
                }
            });

            line
        })
        .map(|line: Line| match line.score {
            Some(score) => score,
            _ => 0,
        })
        .sum::<usize>();

    println!("part 1 - {}", score);

    part_2(data);

    Ok(())
}

fn part_2(data: String) -> () {
    let mut parts: Vec<Vec<String>> = vec![Vec::new()];

    data.lines().enumerate().for_each(|(i, x)| {
        // Make sure a new vector is ready to push to
        if (i > 0) && (i % 3 == 0) {
            parts.push(Vec::new());
        }

        let current_length = parts.len();
        parts[current_length - 1].push(x.to_string());
    });

    let score = parts
        .iter()
        .map(|part: &Vec<String>| {
            if let Some(base_string) = part.first() {
                let mut mutable = base_string.clone();

                part[1..part.len()].iter().for_each(|s| {
                    mutable = s
                        .chars()
                        .filter(|c| mutable.contains(*c))
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join("");
                });

                if let Some(char) = mutable.chars().next() {
                    if let Some(pos) = ALPHABET
                        .chars()
                        .position(|c| c.to_string().eq(&char.to_string()))
                    {
                        // Plus one since index starts at 0
                        return pos + 1;
                    }
                }
            }

            0
        })
        .sum::<usize>();

    println!("part 2 - {}", score);
}
