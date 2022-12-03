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

    Ok(())
}
