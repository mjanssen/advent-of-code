use std::time::Instant;

use day_11::part_2::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start: Instant = Instant::now();

    let file = include_str!("../../input_2.txt");
    let result = process(file);

    println!("{:?}", result);
    println!("Execution time: {:.3?}", start.elapsed());

    Ok(())
}
