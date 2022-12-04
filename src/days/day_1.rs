use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_1.txt")?;

    let mut lines: Vec<u32> = data
        .split("\n\n")
        .map(|x: &str| {
            x.split("\n")
                .map(|x: &str| match x.parse::<u32>() {
                    Ok(v) => v,
                    _ => 0,
                })
                .collect::<Vec<u32>>()
        })
        .map(|nums: Vec<u32>| nums.iter().sum())
        .collect();

    lines.sort();
    println!("part 1 - {:?}", &lines);

    let sum: u32 = lines.iter().rev().take(3).sum();

    println!("part 2 - {:?}", &sum);

    Ok(())
}
