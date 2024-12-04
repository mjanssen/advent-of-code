use std::time::Instant;

use day_02::part_1::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start: Instant = Instant::now();

    let file = include_str!("../../input_1.txt");
    let result = process(file);

    println!("{:?}", result);
    println!("Execution time: {:.3?}", start.elapsed());

    Ok(())
}
